use super::StringError;
use crate::server::{
    entities::{member, prelude::*},
    handlers::Response,
    state::AppState,
    strings,
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

/// Register a new user with the specified username and password.
pub async fn register(
    State(state): State<Arc<AppState>>,
    body: Result<Json<Registration>, JsonRejection>,
) -> Result<impl IntoResponse, axum::response::Response> {
    let Json(Registration { username, password }) = body.map_err(|e| {
        StringError(
            e.body_text().replace(
                "Failed to deserialize the JSON body into the target type: ",
                "",
            ),
            StatusCode::BAD_REQUEST,
        )
    })?;
    // Ensure that the username is at least 3 characters long. Totally arbitrary.
    if username.len() < 3 {
        return Err(
            StringError(strings::USERNAME_TOO_SHORT.into(), StatusCode::BAD_REQUEST).into(),
        );
    }
    // Make sure that the password is at least 8 characters long. Protects against grossly insecure
    // passwords while not being too annoying for uncaring users.
    if password.len() < 8 {
        return Err(
            StringError(strings::PASSWORD_TOO_SHORT.into(), StatusCode::BAD_REQUEST).into(),
        );
    }
    // Ensure that the password contains at least one alphabetic character and one number.
    // An additional check to ensure the password isn't horribly insecure.
    match password {
        _ if !password.contains(|c: char| c.is_alphabetic()) => {
            return Err(
                StringError(strings::PASSWORD_NO_ALPHA.into(), StatusCode::BAD_REQUEST).into(),
            );
        }
        _ if !password.contains(|c: char| c.is_numeric()) => {
            return Err(
                StringError(strings::PASSWORD_NO_NUMERIC.into(), StatusCode::BAD_REQUEST).into(),
            );
        }
        _ => {}
    }
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
    Ok(Response::new(
        json!({"id": model.last_insert_id.to_string() }),
        StatusCode::CREATED,
    ))
}

#[cfg(test)]
mod tests {
    use crate::server::{self, handlers::Response};
    use axum::http::StatusCode;
    use test_utils::function;

    #[tokio::test]
    async fn taken() {
        let database = sea_orm::Database::connect(server::INSECURE_DEFAULT_DATABASE_URL)
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        let client = test_utils::Client::authenticated(&[&function!()], &url, true).await;
        let credentials = serde_json::json!({
            "username": function!(),
            "password": function!()
        });
        let resp: Response<String> = client.post(&url, "/register", credentials).await;
        assert_eq!(resp.code, StatusCode::CONFLICT);
        assert_eq!(resp.message, "that username is already taken");
    }

    #[tokio::test]
    async fn success() {
        let database = sea_orm::Database::connect(server::INSECURE_DEFAULT_DATABASE_URL)
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        let client = test_utils::Client::new();
        let credentials = serde_json::json!({
            "username": function!(),
            "password": function!()
        });
        let resp: Response<test_utils::Map> = client.post(&url, "/register", credentials).await;
        assert_eq!(resp.code, StatusCode::CREATED);
    }
}
