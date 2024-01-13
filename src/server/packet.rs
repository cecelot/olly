use crate::{
    server::{
        entities::{member, prelude::*, session},
        errors,
        state::AppState,
    },
    Game, Piece,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::ws::Message, http::StatusCode};
use base64::Engine;
use rand::RngCore;
use sea_orm::{sea_query::OnConflict, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{de::Error, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    op: Opcode,
    #[serde(flatten)]
    data: Option<Data>,
    s: Option<String>,
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
        match packet.op {
            op if !matches!(op, Opcode::Create) && packet.data.is_none() => {
                Err(ParseError::Json(serde_json::Error::missing_field("d")))
            }
            op if !matches!(op, Opcode::Identify) && packet.s.is_none() => {
                Err(ParseError::Json(serde_json::Error::missing_field("s")))
            }
            _ => Ok(packet),
        }
    }
}

impl Packet {
    pub async fn process(
        &self,
        state: &AppState,
        sender: Option<mpsc::Sender<Response>>,
    ) -> Response {
        match self.op {
            Opcode::Identify => self.identify(state).await,
            Opcode::Create => self.create(state).await,
            Opcode::Place => self.place(state).await,
            Opcode::Reset => todo!(),
            Opcode::Join => self.join(state, sender.expect("missing sender")).await,
            Opcode::Leave => todo!(),
        }
        .unwrap_or_else(std::convert::identity)
    }

    async fn identify(&self, state: &AppState) -> Result<Response, Response> {
        let (username, password) = match self.data.as_ref().unwrap() {
            Data::Identify { username, password } => (username, password),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        let user = match Member::find()
            .filter(member::Column::Username.eq(username))
            .one(state.database.as_ref())
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(Response::error(
                    errors::INVALID_USERNAME,
                    StatusCode::NOT_FOUND,
                ))
            }
            Err(e) => {
                return Err(Response::error(
                    &e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        };
        let hashed = PasswordHash::new(user.password.as_str()).map_err(|_| {
            Response::error(errors::INVALID_PASSWORD_FORMAT, StatusCode::BAD_REQUEST)
        })?;
        if Argon2::default()
            .verify_password(password.as_bytes(), &hashed)
            .is_err()
        {
            return Err(Response::error(
                errors::INVALID_PASSWORD,
                StatusCode::FORBIDDEN,
            ));
        }
        let key = {
            let mut dst = [0; 32];
            // ThreadRng satisfies the CryptoRng trait, so
            // it should be cryptographically secure. TODO:
            // look into a more secure key generation method.
            rand::thread_rng().fill_bytes(&mut dst);
            base64::prelude::BASE64_STANDARD.encode(dst)
        };
        let token = match Session::insert(session::ActiveModel {
            id: ActiveValue::set(user.id),
            key: ActiveValue::set(key.clone()),
        })
        .on_conflict(
            OnConflict::column(session::Column::Id)
                .update_column(session::Column::Key)
                .value(session::Column::Key, key.clone())
                .to_owned(),
        )
        .exec(state.database.as_ref())
        .await
        {
            Ok(_) => key,
            Err(e) => {
                return Err(Response::error(
                    &e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        };
        Ok(Response::Ready { token })
    }

    async fn create(&self, state: &AppState) -> Result<Response, Response> {
        let game = Game::new();
        let (tx, _) = broadcast::channel(16);
        let id = Uuid::now_v7();
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        games.insert(id, game);
        rooms.insert(id, tx);
        Ok(Response::Created { id: id.to_string() })
    }

    async fn join(
        &self,
        state: &AppState,
        sender: mpsc::Sender<Response>,
    ) -> Result<Response, Response> {
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
        // Send the current state of the room.
        let games = state.games.lock().expect("mutex was poisoned");
        let game = games.get(&uuid).ok_or(Response::error(
            errors::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        // Spawn a task to listen for room updates to broadcast.
        tokio::spawn(async move {
            while let Ok(update) = rx.recv().await {
                let _ = sender.send(update).await;
            }
        });
        Ok(Response::State(game.clone()))
    }

    async fn place(&self, state: &AppState) -> Result<Response, Response> {
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
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
        let tx = rooms.get_mut(&uuid).ok_or(Response::error(
            errors::INVALID_GAME_ID,
            StatusCode::NOT_FOUND,
        ))?;
        let res = game.place(x, y, piece).map_or_else(
            |e| Err(Response::error(&e.to_string(), StatusCode::BAD_REQUEST)),
            |_| Ok(Response::Ok),
        )?;
        let _ = tx.send(Response::State(game.clone()));
        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Error { message: String, code: u16 },
    Created { id: String },
    Ready { token: String },
    State(Game),
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
