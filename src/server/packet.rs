use crate::{
    server::{
        entities::{game, prelude::Game as GameModel},
        handlers::StringError,
        helpers,
        state::AppState,
        strings,
    },
    Game, Piece,
};
use axum::{extract::ws::Message, http::StatusCode};
use futures::Future;
use redis::Commands;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    op: Opcode,
    d: Data,
    t: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Data {
    Identify,
    Place {
        id: String,
        x: usize,
        y: usize,
        piece: Piece,
    },
    Create {
        guest: String,
    },
    Join {
        id: String,
    },
    Leave {
        id: String,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum Opcode {
    // Create = 1 << 0
    Place = 1 << 1,
    Join,
    Leave,
    Ended,
    Identify,
    Preview,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid utf-8")]
    InvalidUtf8,
    #[error("{0}")]
    Json(serde_json::Error),
}

impl TryFrom<&Message> for Packet {
    type Error = ParseError;

    fn try_from(msg: &Message) -> Result<Self, Self::Error> {
        let s = msg.to_text().map_err(|_| ParseError::InvalidUtf8)?;
        let packet: Self = serde_json::from_str(s).map_err(ParseError::Json)?;
        Ok(packet)
    }
}

impl Packet {
    pub async fn process(&self, state: &AppState, sender: Option<mpsc::Sender<Event>>) -> Event {
        match self.op {
            Opcode::Identify => self.identify(state).await,
            Opcode::Place => self.authenticated(state, |p| p.place(state)).await,
            Opcode::Preview => self.authenticated(state, |p| p.preview(state)).await,
            Opcode::Join => {
                self.authenticated(state, |p| p.join(state, sender.expect("missing sender")))
                    .await
            }
            Opcode::Leave => self.authenticated(state, |p| p.leave(state)).await,
            Opcode::Ended => todo!(),
        }
        .unwrap_or_else(std::convert::identity)
    }

    async fn identify(&self, state: &AppState) -> Result<Event, Event> {
        // Verify that the token is valid.
        self.current_user(state).await?;
        Ok(Event::new(EventKind::Ready, EventData::Ready))
    }

    async fn join(&self, state: &AppState, sender: mpsc::Sender<Event>) -> Result<Event, Event> {
        let Data::Join { id } = &self.d else {
            panic!("expected serde to reject invalid packet data")
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(strings::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        // Subscribe to the broadcast channel for the specified room.
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        let mut rx = rooms
            .get_mut(&uuid)
            .ok_or(Event::error(
                strings::INVALID_GAME_ID,
                StatusCode::NOT_FOUND,
            ))?
            .subscribe();
        // Send the current state of the room.
        let games = state.games.lock().expect("mutex was poisoned");
        let game = games.get(&uuid).ok_or(Event::error(
            strings::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        // Spawn a task to listen for room updates to broadcast.
        tokio::spawn(async move {
            while let Ok(update) = rx.recv().await {
                let _ = sender.send(update).await;
            }
        });
        Ok(Event::new(
            EventKind::GameUpdate,
            EventData::GameUpdate { game: game.clone() },
        ))
    }

    async fn leave(&self, state: &AppState) -> Result<Event, Event> {
        let Data::Leave { id } = &self.d else {
            panic!("expected serde to reject invalid packet data")
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(strings::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        // Delete the game from the database.
        GameModel::delete_by_id(uuid)
            .exec(state.database.as_ref())
            .await
            .map_err(|e| Event::error(&e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        let tx = rooms.get_mut(&uuid).ok_or(Event::error(
            strings::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        let _ = tx.send(Event::new(EventKind::GameAbort, EventData::GameAbort));
        // Delete game and room from global state.
        let mut games = state.games.lock().expect("mutex was poisoned");
        games.remove(&uuid).ok_or(Event::error(
            strings::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        rooms.remove(&uuid).unwrap();
        Ok(Event::new(EventKind::Ack, EventData::Ack))
    }

    async fn place(&self, state: &AppState) -> Result<Event, Event> {
        let Data::Place { id, x, y, piece } = &self.d else {
            panic!("expected serde to reject invalid packet data")
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(strings::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        let game = games.get_mut(&uuid).ok_or(Event::error(
            strings::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        let tx = rooms.get_mut(&uuid).ok_or(Event::error(
            strings::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        let res = game.place(*x, *y, *piece).map_or_else(
            |e| Err(Event::error(&e.to_string(), StatusCode::BAD_REQUEST)),
            |()| Ok(Event::new(EventKind::Ack, EventData::Ack)),
        )?;
        let _ = tx.send(Event::new(
            EventKind::GameUpdate,
            EventData::GameUpdate { game: game.clone() },
        ));
        if let Ok(mut conn) = state.redis.get_connection() {
            let _ = conn.set::<String, String, String>(
                format!("game:{}", id.clone()),
                serde_json::to_string(game).unwrap(),
            );
        }
        Ok(res)
    }

    async fn preview(&self, state: &AppState) -> Result<Event, Event> {
        let Data::Place { id, x, y, piece } = &self.d else {
            panic!("expected serde to reject invalid packet data")
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let mut games = state.games.lock().expect("mutex was poisoned");
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(strings::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        let game = games.get_mut(&uuid).ok_or(Event::error(
            strings::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        game.preview(*x, *y, *piece).map_or_else(
            |e| Err(Event::error(&e.to_string(), StatusCode::BAD_REQUEST)),
            |changed| {
                Ok(Event::new(
                    EventKind::GameUpdatePreview,
                    EventData::GameUpdatePreview { changed },
                ))
            },
        )
    }
}

// Middleware to require authentication for chosen Packet types.
impl Packet {
    async fn authenticated<'a, F>(
        &'a self,
        state: &AppState,
        f: impl FnOnce(&'a Self) -> F,
    ) -> Result<Event, Event>
    where
        F: Future<Output = Result<Event, Event>>,
    {
        match self.current_user(state).await {
            Ok(_) => f(self).await,
            Err(e) => Err(e),
        }
    }
}

// A collection of helper functions for performing database operations.
impl Packet {
    async fn current_user(&self, state: &AppState) -> Result<String, Event> {
        helpers::get_session(state, &self.t)
            .await
            .map_err(|StringError(message, code)| Event::error(&message, code))
    }

    async fn game(&self, state: &AppState, id: &str) -> Result<game::Model, Event> {
        let id = Uuid::from_str(id)
            .map_err(|_| Event::error(strings::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        match GameModel::find_by_id(id).one(state.database.as_ref()).await {
            Ok(Some(game)) => Ok(game),
            Ok(None) => Err(Event::error(
                strings::INVALID_GAME_ID,
                StatusCode::NOT_FOUND,
            )),
            Err(e) => Err(Event::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}

// A collection of helper functions for validating data.
impl Packet {
    async fn ensure_participant(&self, state: &AppState, id: &str) -> Result<(), Event> {
        let user = self.current_user(state).await?;
        let game = self.game(state, id).await?;
        if game.host != user && game.guest != user {
            return Err(Event::error(
                strings::INVALID_GAME_ID,
                StatusCode::NOT_FOUND,
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    op: EventKind,
    d: EventData,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EventKind {
    Ack = 1 << 0,
    Ready,
    GameAbort,
    GameUpdate = 1 << 2,
    GameUpdatePreview,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    Ack,
    Ready,
    GameCreate { id: String },
    GameUpdate { game: Game },
    GameUpdatePreview { changed: Vec<(usize, usize)> },
    GameAbort,
    Error { message: String, code: u16 },
}

impl Event {
    pub fn new(op: EventKind, d: EventData) -> Self {
        Self { op, d }
    }

    pub fn error(message: &str, code: StatusCode) -> Self {
        Self {
            op: EventKind::Error,
            d: EventData::Error {
                message: message.to_string(),
                code: code.into(),
            },
        }
    }

    pub fn data(&self) -> &EventData {
        &self.d
    }
}
