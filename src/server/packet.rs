use crate::{
    server::{
        entities::{game, member, prelude as entities, session},
        errors,
        state::AppState,
    },
    Game, Piece,
};
use axum::{extract::ws::Message, http::StatusCode};
use futures::Future;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;
use tokio::sync::{broadcast, mpsc};
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
}

#[derive(Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum Opcode {
    Create = 1 << 0,
    Place,
    Join,
    Leave,
    Reset,
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
            Opcode::Create => self.authenticated(state, |p| p.create(state)).await,
            Opcode::Place => self.authenticated(state, |p| p.place(state)).await,
            Opcode::Preview => self.authenticated(state, |p| p.preview(state)).await,
            Opcode::Join => {
                self.authenticated(state, |p| p.join(state, sender.expect("missing sender")))
                    .await
            }
            Opcode::Leave => todo!(),
            Opcode::Reset => todo!(),
        }
        .unwrap_or_else(std::convert::identity)
    }

    async fn identify(&self, state: &AppState) -> Result<Event, Event> {
        // Verify that the token is valid.
        self.current_user(state).await?;
        Ok(Event::new(EventKind::Ready, EventData::Ready))
    }

    async fn create(&self, state: &AppState) -> Result<Event, Event> {
        let host = self.current_user(state).await?;
        let guest = match &self.d {
            Data::Create { guest } => self.user(state, guest).await?,
            _ => panic!("expected serde to reject invalid packet data"),
        };
        let id = Uuid::now_v7();
        let model = game::ActiveModel {
            id: ActiveValue::set(id),
            host: ActiveValue::set(host),
            guest: ActiveValue::set(guest.id.to_string()),
        };
        if let Err(e) = model.insert(state.database.as_ref()).await {
            return Err(Event::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        };
        let game = Game::new();
        let (tx, _) = broadcast::channel(16);
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        games.insert(id, game);
        rooms.insert(id, tx);
        Ok(Event::new(
            EventKind::GameCreate,
            EventData::GameCreate { id: id.to_string() },
        ))
    }

    async fn join(&self, state: &AppState, sender: mpsc::Sender<Event>) -> Result<Event, Event> {
        let id = match &self.d {
            Data::Join { id } => id,
            _ => panic!("expected serde to reject invalid packet data"),
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        // Subscribe to the broadcast channel for the specified room.
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        let mut rx = rooms
            .get_mut(&uuid)
            .ok_or(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND))?
            .subscribe();
        // Send the current state of the room.
        let games = state.games.lock().expect("mutex was poisoned");
        let game = games
            .get(&uuid)
            .ok_or(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND))?;
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

    async fn place(&self, state: &AppState) -> Result<Event, Event> {
        let (id, &x, &y, &piece) = match &self.d {
            Data::Place { id, x, y, piece } => (id, x, y, piece),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        let game = games
            .get_mut(&uuid)
            .ok_or(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND))?;
        let tx = rooms
            .get_mut(&uuid)
            .ok_or(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND))?;
        let res = game.place(x, y, piece).map_or_else(
            |e| Err(Event::error(&e.to_string(), StatusCode::BAD_REQUEST)),
            |_| Ok(Event::new(EventKind::Ack, EventData::Ack)),
        )?;
        let _ = tx.send(Event::new(
            EventKind::GameUpdate,
            EventData::GameUpdate { game: game.clone() },
        ));
        Ok(res)
    }

    async fn preview(&self, state: &AppState) -> Result<Event, Event> {
        let (id, &x, &y, &piece) = match &self.d {
            Data::Place { id, x, y, piece } => (id, x, y, piece),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let mut games = state.games.lock().expect("mutex was poisoned");
        let uuid = Uuid::from_str(id)
            .map_err(|_| Event::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        let game = games
            .get_mut(&uuid)
            .ok_or(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND))?;
        game.preview(x, y, piece).map_or_else(
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
        let session = entities::Session::find()
            .filter(session::Column::Key.eq(&self.t))
            .one(state.database.as_ref())
            .await;
        match session {
            Ok(Some(session)) => Ok(session.id.to_string()),
            Ok(None) => Err(Event::error(errors::INVALID_TOKEN, StatusCode::FORBIDDEN)),
            Err(e) => Err(Event::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    async fn user(&self, state: &AppState, username: &String) -> Result<member::Model, Event> {
        crate::server::helpers::get_user(state, username)
            .await
            .map_err(|(message, code)| Event::error(&message, code))
    }

    async fn game(&self, state: &AppState, id: &str) -> Result<game::Model, Event> {
        let id = Uuid::from_str(id)
            .map_err(|_| Event::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST))?;
        match entities::Game::find_by_id(id)
            .one(state.database.as_ref())
            .await
        {
            Ok(Some(game)) => Ok(game),
            Ok(None) => Err(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND)),
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
            return Err(Event::error(errors::INVALID_GAME_ID, StatusCode::NOT_FOUND));
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
    GameCreate,
    GameUpdate,
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
