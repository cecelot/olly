use super::StringError;
use crate::{
    server::{entities::game::Column, extractors::User, helpers, state::AppState, strings},
    Game,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{ActiveModelTrait, IntoActiveModel, ModelTrait, Value};
use serde_json::json;
use std::{str::FromStr, sync::Arc};
use tokio::sync::broadcast;
use uuid::Uuid;

/// Retrieve the details for the specified game.
pub async fn game(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response<Body>> {
    // Fetch the user and game from the database.
    let user = helpers::get_user(&state, &user.username, true).await?;
    let game = helpers::get_game(&state, &id).await?;
    // Convert to strings for more ergonomic comparison.
    let authed = user.id.to_string();
    let host = game.host.to_string();
    let guest = game.guest.to_string();
    // Ensure that the authenticated user is either the host or the guest.
    (authed == host || authed == guest)
        // If so, provide the details for the specified game.
        .then(|| {
            Ok(super::Response::new(
                json!({
                    "id": game.id,
                    "pending": game.pending,
                    "host": game.host,
                    "guest": game.guest,
                }),
                StatusCode::OK,
            ))
        })
        // Otherwise, pretend the game does not exist.
        .unwrap_or_else(|| {
            Err(StringError(strings::INVALID_GAME_ID.into(), StatusCode::NOT_FOUND).into_response())
        })
}

pub async fn cancel(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response<Body>> {
    // Fetch the user and game from the database.
    let user = helpers::get_user(&state, &user.username, true).await?;
    let game = helpers::get_game(&state, &id).await?;
    // Convert to strings for more ergonomic comparison.
    let authed = user.id.to_string();
    let host = game.host.to_string();
    // Ensure that the authenticated user is the host.
    if authed == host {
        // If so, delete the game record from the database.
        let game = helpers::get_game(&state, &id).await?;
        game.delete(state.database.as_ref())
            .await
            .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(super::Response::new(json!({}), StatusCode::NO_CONTENT))
    } else {
        // Otherwise, pretend the game does not exist.
        Err(StringError(strings::INVALID_GAME_ID.into(), StatusCode::NOT_FOUND).into_response())
    }
}

pub async fn accept(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response<Body>> {
    // Fetch the user and game from the database.
    let user = helpers::get_user(&state, &user.username, true).await?;
    let game = helpers::get_game(&state, &id).await?;
    // Convert to strings for more ergonomic comparison.
    let authed = user.id.to_string();
    let guest = game.guest.to_string();
    // Ensure that the authenticated user is the guest.
    if authed == guest {
        // If so, update the game record to indicate that the game is no longer pending.
        let game = helpers::get_game(&state, &id).await?;
        let mut active = game.into_active_model();
        active.set(Column::Pending, Value::Bool(Some(false)));
        active
            .save(state.database.as_ref())
            .await
            .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
        // Create a new game object and broadcast channel for notifications to websocket
        // subscribers.
        let game = Game::new();
        let gid = Uuid::from_str(&id).unwrap();
        let (tx, _) = broadcast::channel(16);
        // Insert the game object and broadcast channel into the global state.
        let mut games = state.games.lock().expect("mutex was poisoned");
        let mut rooms = state.rooms.lock().expect("mutex was poisoned");
        games.insert(gid, game);
        rooms.insert(gid, tx);
        Ok(super::Response::new(json!({}), StatusCode::OK))
    } else {
        // Otherwise, pretend the game does not exist.
        Err(StringError(strings::INVALID_GAME_ID.into(), StatusCode::NOT_FOUND).into_response())
    }
}

pub async fn decline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response<Body>> {
    // Fetch the user and game from the database.
    let user = helpers::get_user(&state, &user.username, true).await?;
    let game = helpers::get_game(&state, &id).await?;
    // Convert to strings for more ergonomic comparison.
    let authed = user.id.to_string();
    let guest = game.guest.to_string();
    // Ensure that the authenticated user is the guest.
    if authed == guest {
        // If so, delete the game record from the database.
        let game = helpers::get_game(&state, &id).await?;
        game.delete(state.database.as_ref())
            .await
            .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(super::Response::new(json!({}), StatusCode::OK))
    } else {
        // Otherwise, pretend the game does not exist.
        Err(StringError(strings::INVALID_GAME_ID.into(), StatusCode::NOT_FOUND).into_response())
    }
}
