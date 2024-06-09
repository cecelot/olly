use super::StringError;
use crate::server::{extractors::User, helpers, state::AppState, strings::INVALID_GAME_ID};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    id: String,
}

pub async fn game(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response<Body>> {
    let user = helpers::get_user(&state, &user.username, true).await?;
    let game = helpers::get_game(&state, &id).await?;
    let authed = user.id.to_string();
    let host = game.host.to_string();
    let guest = game.guest.to_string();
    (authed == host || authed == guest)
        .then(|| {
            Ok(super::Response::new(
                json!({
                    "id": game.id,
                    "host": game.host,
                    "guest": game.guest,
                }),
                StatusCode::OK,
            ))
        })
        .unwrap_or_else(|| {
            Err(StringError(INVALID_GAME_ID.into(), StatusCode::NOT_FOUND).into_response())
        })
}
