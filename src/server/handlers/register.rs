use crate::server::{
    entities::{member, prelude::*},
    errors,
    state::AppState,
    Response,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sea_orm::{ActiveValue, DbErr, EntityTrait, RuntimeErr};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    username: String,
    password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    body: Json<Registration>,
) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let password = body.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = match argon2.hash_password(password, &salt) {
        Ok(hashed) => hashed.to_string(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::error(
                    &e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            )
        }
    };
    let registration = member::ActiveModel {
        id: ActiveValue::set(id),
        username: ActiveValue::set(body.username.clone()),
        password: ActiveValue::set(hashed),
    };
    let model = Member::insert(registration)
        .exec(state.database.as_ref())
        .await;
    match model {
        Ok(model) => (
            StatusCode::CREATED,
            Json(Response::Created {
                id: model.last_insert_id.to_string(),
            }),
        ),
        Err(e) => match e {
            DbErr::Exec(RuntimeErr::SqlxError(e))
                if e.as_database_error()
                    .is_some_and(|e| e.code().is_some_and(|code| code == "23505")) =>
            {
                (
                    StatusCode::CONFLICT,
                    Json(Response::error(
                        errors::USERNAME_TAKEN,
                        StatusCode::CONFLICT,
                    )),
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::error(
                    &e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            ),
        },
    }
}
