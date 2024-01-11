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
    pub fn process(&self, state: &AppState) -> Response {
        match self.op {
            Opcode::Create => self.create(state),
            Opcode::Place => self.place(state),
            Opcode::Reset => todo!(),
            Opcode::Join => todo!(),
            Opcode::Leave => todo!(),
        }
        .unwrap_or_else(std::convert::identity)
    }

    fn create(&self, state: &AppState) -> Result<Response, Response> {
        let game = Game::new();
        let id = Uuid::new_v7(Timestamp::now(context::NoContext));
        let mut games = state.games.lock().expect("mutex was poisoned");
        games.insert(id, game);
        Ok(Response::Created { id: id.to_string() })
    }

    fn place(&self, state: &AppState) -> Result<Response, Response> {
        let mut games = state.games.lock().expect("mutex was poisoned");
        let (id, &x, &y, &piece) = match self.data.as_ref().unwrap() {
            Data::Place { id, x, y, piece } => (id, x, y, piece),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        let uuid = Uuid::from_str(id).map_err(|_| Response::Error {
            message: "invalid game id format".into(),
            code: StatusCode::BAD_REQUEST.into(),
        })?;
        let game = games.get_mut(&uuid).ok_or(Response::Error {
            message: format!("no game with id {}", id),
            code: StatusCode::NOT_FOUND.into(),
        })?;
        game.place(x, y, piece).map_or_else(
            |e| {
                Err(Response::Error {
                    message: e.to_string(),
                    code: StatusCode::BAD_REQUEST.into(),
                })
            },
            |_| Ok(Response::State(game.clone())),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Error { message: String, code: u16 },
    Created { id: String },
    State(Game),
    Ok,
}
