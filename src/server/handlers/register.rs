use crate::server::{
    entities::{member, prelude::*},
    errors,
    state::AppState,
    HttpResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveValue, DbErr, EntityTrait, RuntimeErr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    username: String,
    password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    body: Result<Json<Registration>, JsonRejection>,
) -> impl IntoResponse {
    let (username, password) = match body {
        Ok(body) => (body.username.clone(), body.password.clone()),
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(HttpResponse::new(
                    e.body_text().replace(
                        "Failed to deserialize the JSON body into the target type: ",
                        "",
                    ),
                    StatusCode::BAD_REQUEST,
                )),
            )
        }
    };
    let id = Uuid::now_v7();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hashed) => hashed.to_string(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HttpResponse::new(
                    e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            )
        }
    };
    let registration = member::ActiveModel {
        id: ActiveValue::set(id),
        username: ActiveValue::set(username),
        password: ActiveValue::set(hashed),
    };
    let model = Member::insert(registration)
        .exec(state.database.as_ref())
        .await;
    match model {
        Ok(model) => (
            StatusCode::CREATED,
            Json(HttpResponse::new(
                json!({"id": model.last_insert_id.to_string() }),
                StatusCode::CREATED,
            )),
        ),
        Err(e) => match e {
            DbErr::Exec(RuntimeErr::SqlxError(e))
                if e.as_database_error()
                    .is_some_and(|e| e.code().is_some_and(|code| code == "23505")) =>
            {
                (
                    StatusCode::CONFLICT,
                    Json(HttpResponse::new(
                        errors::USERNAME_TAKEN,
                        StatusCode::CONFLICT,
                    )),
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HttpResponse::new(
                    e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            ),
        },
    }
}
