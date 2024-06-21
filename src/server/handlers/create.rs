use super::StringError;
use crate::server::{entities::game, extractors::User, helpers, state::AppState};
use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRequest {
    guest: String,
}

/// Create a new game with the specified host and guest.
pub async fn create(
    State(state): State<Arc<AppState>>,
    host: User,
    Json(body): Json<GameRequest>,
) -> Result<impl IntoResponse, Response<Body>> {
    // Fetch the user objects associated with the host and guest usernames to
    // ensure that they exist.
    let host = helpers::get_user(&state, &host.username, true).await?;
    let guest = helpers::get_user(&state, &body.guest, true).await?;
    // Create a new game record and insert it into the database.
    let id = Uuid::now_v7();
    let model = game::ActiveModel {
        id: ActiveValue::set(id),
        host: ActiveValue::set(host.id.to_string()),
        guest: ActiveValue::set(guest.id.to_string()),
        pending: ActiveValue::set(true),
    };
    model
        .insert(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(super::Response::new(
        json!({
            "id": id,
            "host": host.id,
            "guest": guest.id,
            "pending": true
        }),
        StatusCode::CREATED,
    ))
}
