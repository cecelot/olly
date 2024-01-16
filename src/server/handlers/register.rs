use crate::server::{
    entities::{member, prelude::*},
    state::AppState,
    strings, HttpResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{ActiveValue, DbErr, EntityTrait, RuntimeErr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use super::StringError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    username: String,
    password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    body: Result<Json<Registration>, JsonRejection>,
) -> Result<impl IntoResponse, Response> {
    let Json(Registration { username, password }) = body.map_err(|e| {
        StringError(
            e.body_text().replace(
                "Failed to deserialize the JSON body into the target type: ",
                "",
            ),
            StatusCode::BAD_REQUEST,
        )
    })?;
    let id = Uuid::now_v7();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| {
            StringError(
                strings::INVALID_PASSWORD_FORMAT.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .map(|hashed| hashed.to_string())?;
    let registration = member::ActiveModel {
        id: ActiveValue::set(id),
        username: ActiveValue::set(username),
        password: ActiveValue::set(hashed),
    };
    let model = Member::insert(registration)
        .exec(state.database.as_ref())
        .await;
    let model = model.map_err(|e| match e {
        DbErr::Exec(RuntimeErr::SqlxError(e))
            if e.as_database_error()
                .is_some_and(|e| e.code().is_some_and(|code| code == "23505")) =>
        {
            StringError(strings::USERNAME_TAKEN.into(), StatusCode::CONFLICT)
        }
        _ => StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
    })?;
    Ok(HttpResponse::new(
        json!({"id": model.last_insert_id.to_string() }),
        StatusCode::CREATED,
    ))
}
