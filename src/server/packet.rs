use crate::{server::state::AppState, Game, Piece};
use axum::{extract::ws::Message, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::{timestamp::context, Timestamp, Uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    op: Opcode,
    data: Option<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Data {
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

#[derive(Debug, Serialize, Deserialize)]
enum Opcode {
    Create,
    Join,
    Place,
    Reset,
    Leave,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Message is not valid UTF-8")]
    InvalidUtf8,
    #[error("Invalid JSON: {0}")]
    Json(serde_json::Error),
}

impl TryFrom<Message> for Packet {
    type Error = ParseError;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        let s = msg.to_text().map_err(|_| ParseError::InvalidUtf8)?;
        let packet = serde_json::from_str(s).map_err(ParseError::Json)?;
        Ok(packet)
    }
}

impl Packet {
    pub fn process(&self, state: &AppState) -> Reply {
        match self.op {
            Opcode::Create => self.create(state),
            Opcode::Place => self.place(state),
            Opcode::Reset => todo!(),
            Opcode::Join => todo!(),
            Opcode::Leave => todo!(),
        }
    }

    fn create(&self, state: &AppState) -> Reply {
        let game = Game::new();
        let id = Uuid::new_v7(Timestamp::now(context::NoContext));
        let mut games = state.games.lock().expect("mutex was poisoned");
        games.insert(id, game);
        Reply::Created { id: id.to_string() }
    }

    fn place(&self, state: &AppState) -> Reply {
        let mut games = state.games.lock().expect("mutex was poisoned");
        let (id, &x, &y, &piece) = match self.data.as_ref().unwrap() {
            Data::Place { id, x, y, piece } => (id, x, y, piece),
            _ => panic!("expected serde reject invalid packet data"),
        };
        let game = games.get_mut(&Uuid::from_str(id).unwrap()).unwrap();
        if let Err(e) = game.place(x, y, piece) {
            Reply::Error {
                message: e.to_string(),
                code: StatusCode::BAD_REQUEST.into(),
            }
        } else {
            Reply::State(game.clone())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    Error { message: String, code: u16 },
    Created { id: String },
    State(Game),
    Ok,
}
