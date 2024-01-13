use crate::{
    server::{entities::member, errors, state::AppState},
    Game, Piece,
};
use axum::{extract::ws::Message, http::StatusCode};
use serde::{de::Error, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;
use tokio::sync::{broadcast, mpsc};
use uuid::{timestamp::context, Timestamp, Uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    op: Opcode,
    #[serde(flatten)]
    data: Option<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
enum Data {
    Identify {
        username: String,
        password: String,
    },
    Place {
        id: String,
        x: usize,
        y: usize,
        piece: Piece,
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
        if packet.op != Opcode::Create && packet.data.is_none() {
            Err(ParseError::Json(serde_json::Error::missing_field("d")))
        } else {
            Ok(packet)
        }
    }
}

impl Packet {
    pub fn process(&self, state: &AppState, sender: Option<mpsc::Sender<Response>>) -> Response {
        match self.op {
            Opcode::Identify => self.identify(state),
            Opcode::Create => self.create(state),
            Opcode::Place => self.place(state),
            Opcode::Reset => todo!(),
            Opcode::Join => self.join(state, sender.expect("missing sender")),
            Opcode::Leave => todo!(),
        }
        .unwrap_or_else(std::convert::identity)
    }

    fn identify(&self, _: &AppState) -> Result<Response, Response> {
        // TODO: Actually authenticate the user.
        Ok(Response::Ready {
            token: {
                let id = Uuid::new_v7(Timestamp::now(context::NoContext));
                id.to_string()
            },
        })
    }

    fn create(&self, state: &AppState) -> Result<Response, Response> {
        let game = Game::new();
        let (tx, _) = broadcast::channel(16);
        let id = Uuid::new_v7(Timestamp::now(context::NoContext));
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        games.insert(id, game);
        rooms.insert(id, tx);
        Ok(Response::Created { id: id.to_string() })
    }

    fn join(&self, state: &AppState, sender: mpsc::Sender<Response>) -> Result<Response, Response> {
        let id = match self.data.as_ref().unwrap() {
            Data::Join { id } => id,
            _ => panic!("expected serde to reject invalid packet data"),
        };
        let uuid = Uuid::from_str(id).map_err(|_| {
            Response::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST)
        })?;
        // Subscribe to the broadcast channel for the specified room.
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        let mut rx = rooms
            .get_mut(&uuid)
            .ok_or(Response::error(
                errors::INVALID_GAME_ID,
                StatusCode::NOT_FOUND,
            ))?
            .subscribe();
        // Spawn a task to listen for room updates to broadcast.
        tokio::spawn(async move {
            while let Ok(update) = rx.recv().await {
                sender.send(update).await.unwrap();
            }
        });
        Ok(Response::Ok)
    }

    fn place(&self, state: &AppState) -> Result<Response, Response> {
        let mut games = state.games.lock().expect("mutex was poisoned");
        let (id, &x, &y, &piece) = match self.data.as_ref().unwrap() {
            Data::Place { id, x, y, piece } => (id, x, y, piece),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        let uuid = Uuid::from_str(id).map_err(|_| {
            Response::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST)
        })?;
        let game = games.get_mut(&uuid).ok_or(Response::error(
            errors::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        // TODO: Broadcast the update to the room.
        game.place(x, y, piece).map_or_else(
            |e| Err(Response::error(&e.to_string(), StatusCode::BAD_REQUEST)),
            |_| Ok(Response::State(game.clone())),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Error { message: String, code: u16 },
    Created { id: String },
    Ready { token: String },
    State(Game),
    Member(member::Model),
    Ok,
}

impl Response {
    pub fn error(message: &str, code: StatusCode) -> Self {
        Self::Error {
            message: message.to_string(),
            code: code.into(),
        }
    }
}
