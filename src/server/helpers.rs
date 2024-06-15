use crate::server::{
    entities::{game, member, prelude::*, session},
    handlers::StringError,
    strings, AppState, PasswordHash, StatusCode,
};
use argon2::{Argon2, PasswordVerifier};
use sea_orm::{sea_query::OnConflict, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

/// Hashes a password string.
fn hash(s: &str) -> Result<PasswordHash, StringError> {
    PasswordHash::new(s).map_err(|_| {
        StringError(
            strings::INVALID_PASSWORD_FORMAT.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

/// Fetch a user by their username or ID.
pub async fn get_user(
    state: &AppState,
    s: &str,
    username: bool,
) -> Result<member::Model, StringError> {
    let query = if username {
        Member::find().filter(member::Column::Username.eq(s))
    } else {
        Member::find_by_id(Uuid::try_from(s).unwrap())
    };
    match query.one(state.database.as_ref()).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(StringError(
            strings::INVALID_USERNAME.to_string(),
            StatusCode::NOT_FOUND,
        )),
        Err(e) => Err(StringError(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

/// Fetch a game by its ID.
pub async fn get_game(state: &AppState, id: &str) -> Result<game::Model, StringError> {
    let id = Uuid::parse_str(id).map_err(|_| {
        StringError(
            strings::INVALID_GAME_ID.to_string(),
            StatusCode::BAD_REQUEST,
        )
    })?;
    match Game::find_by_id(id).one(state.database.as_ref()).await {
        Ok(Some(game)) => Ok(game),
        Ok(None) => Err(StringError(
            strings::INVALID_GAME_ID.to_string(),
            StatusCode::NOT_FOUND,
        )),
        Err(e) => Err(StringError(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

/// Fetch an authentication session by its token.
pub async fn get_session(state: &AppState, token: &str) -> Result<String, StringError> {
    match Session::find()
        .filter(session::Column::Key.eq(token))
        .one(state.database.as_ref())
        .await
    {
        Ok(Some(session)) => Ok(session.id.to_string()),
        Ok(None) => Err(StringError(
            strings::INVALID_TOKEN.into(),
            StatusCode::FORBIDDEN,
        )),
        Err(e) => Err(StringError(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

/// Create a new authentication session for the specified user.
pub async fn create_session(
    state: &AppState,
    user: &member::Model,
    key: String,
) -> Result<String, StringError> {
    Session::insert(session::ActiveModel {
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
    .map_or_else(
        |e| {
            Err(StringError(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        },
        |_| Ok(key),
    )
}

/// Delete an authentication session by its token.
pub async fn delete_session(state: &AppState, token: String) -> Result<(), StringError> {
    match Session::delete_many()
        .filter(session::Column::Key.eq(token))
        .exec(state.database.as_ref())
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(StringError(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

/// Verifies that the provided password matches the actual password.
pub fn ensure_valid_password(actual: &str, provided: &str) -> Result<(), StringError> {
    let hashed = hash(actual)?;
    Argon2::default()
        .verify_password(provided.as_bytes(), &hashed)
        .map_err(|_| StringError(strings::INVALID_PASSWORD.to_string(), StatusCode::FORBIDDEN))
}
