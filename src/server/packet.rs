use crate::{
    server::{
        entities::{game, member, prelude as entities, session},
        errors,
        state::AppState,
    },
    Game, Piece,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::ws::Message, http::StatusCode};
use base64::Engine;
use futures::Future;
use rand::RngCore;
use sea_orm::{
    sea_query::OnConflict, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter,
};
use serde::{de::Error, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    op: Opcode,
    d: Data,
    s: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
            Opcode::Create => self.authenticated(state, |p| p.create(state)).await,
            Opcode::Place => self.authenticated(state, |p| p.place(state)).await,
            Opcode::Join => {
                self.authenticated(state, |p| p.join(state, sender.expect("missing sender")))
                    .await
            }
            Opcode::Leave => todo!(),
            Opcode::Reset => todo!(),
        }
        .unwrap_or_else(std::convert::identity)
    }

    async fn identify(&self, state: &AppState) -> Result<Response, Response> {
        let (username, password) = match &self.d {
            Data::Identify { username, password } => (username, password),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        let user = self.user(state, username).await?;
        self.ensure_valid_password(&user, password).await?;
        // Generate a random key to use as the session token.
        let key = {
            let mut dst = [0; 32];
            // ThreadRng satisfies the CryptoRng trait, so
            // it should be cryptographically secure. TODO:
            // look into a more secure key generation method.
            rand::thread_rng().fill_bytes(&mut dst);
            base64::prelude::BASE64_STANDARD.encode(dst)
        };
        let token = self.create_session(state, &user, key).await?;
        Ok(Response::Ready { token })
    }

    async fn create(&self, state: &AppState) -> Result<Response, Response> {
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
            return Err(Response::error(
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
        Ok(Response::Created { id: id.to_string() })
    }

    async fn join(
        &self,
        state: &AppState,
        sender: mpsc::Sender<Response>,
    ) -> Result<Response, Response> {
        let id = match &self.d {
            Data::Join { id } => id,
            _ => panic!("expected serde to reject invalid packet data"),
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
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
        let (id, &x, &y, &piece) = match &self.d {
            Data::Place { id, x, y, piece } => (id, x, y, piece),
            _ => panic!("expected serde to reject invalid packet data"),
        };
        // Verify that the authenticated user is either the host or guest of the game.
        self.ensure_participant(state, id).await?;
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
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

// Middleware to require authentication for chosen Packet types.
impl Packet {
    async fn authenticated<'a, F>(
        &'a self,
        state: &AppState,
        f: impl FnOnce(&'a Self) -> F,
    ) -> Result<Response, Response>
    where
        F: Future<Output = Result<Response, Response>>,
    {
        match self.current_user(state).await {
            Ok(_) => f(self).await,
            Err(e) => Err(e),
        }
    }
}

// A collection of helper functions for performing database operations.
impl Packet {
    async fn create_session(
        &self,
        state: &AppState,
        user: &member::Model,
        key: String,
    ) -> Result<String, Response> {
        match entities::Session::insert(session::ActiveModel {
            id: ActiveValue::set(user.id),
            key: ActiveValue::set(key.clone()),
        })
        .on_conflict(
            OnConflict::column(session::Column::Id)
                .update_column(session::Column::Key)
                .to_owned(),
        )
        .exec(state.database.as_ref())
        .await
        {
            Ok(_) => Ok(key),
            Err(e) => Err(Response::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    async fn current_user(&self, state: &AppState) -> Result<String, Response> {
        let token = self.s.as_ref().unwrap();
        let session = entities::Session::find()
            .filter(session::Column::Key.eq(token))
            .one(state.database.as_ref())
            .await;
        match session {
            Ok(Some(session)) => Ok(session.id.to_string()),
            Ok(None) => Err(Response::error(
                errors::INVALID_TOKEN,
                StatusCode::FORBIDDEN,
            )),
            Err(e) => Err(Response::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    async fn user(&self, state: &AppState, username: &String) -> Result<member::Model, Response> {
        match entities::Member::find()
            .filter(member::Column::Username.eq(username))
            .one(state.database.as_ref())
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(Response::error(
                errors::INVALID_USERNAME,
                StatusCode::NOT_FOUND,
            )),
            Err(e) => Err(Response::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    async fn game(&self, state: &AppState, id: &str) -> Result<game::Model, Response> {
        let id = Uuid::from_str(id).map_err(|_| {
            Response::error(errors::INVALID_GAME_ID_FORMAT, StatusCode::BAD_REQUEST)
        })?;
        match entities::Game::find_by_id(id)
            .one(state.database.as_ref())
            .await
        {
            Ok(Some(game)) => Ok(game),
            Ok(None) => Err(Response::error(
                errors::INVALID_GAME_ID,
                StatusCode::NOT_FOUND,
            )),
            Err(e) => Err(Response::error(
                &e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}

// A collection of helper functions for validating data.
impl Packet {
    async fn ensure_valid_password(
        &self,
        user: &member::Model,
        password: &str,
    ) -> Result<(), Response> {
        let hashed = PasswordHash::new(user.password.as_str()).map_err(|_| {
            Response::error(errors::INVALID_PASSWORD_FORMAT, StatusCode::BAD_REQUEST)
        })?;
        if Argon2::default()
            .verify_password(password.as_bytes(), &hashed)
            .is_err()
        {
            Err(Response::error(
                errors::INVALID_PASSWORD,
                StatusCode::FORBIDDEN,
            ))
        } else {
            Ok(())
        }
    }

    async fn ensure_participant(&self, state: &AppState, id: &str) -> Result<(), Response> {
        let user = self.current_user(state).await?;
        let game = self.game(state, id).await?;
        if game.host != user && game.guest != user {
            return Err(Response::error(
                errors::INVALID_GAME_ID,
                StatusCode::NOT_FOUND,
            ));
        }
        Ok(())
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
