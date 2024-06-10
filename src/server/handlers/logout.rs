use crate::server::{helpers, state::AppState, strings};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

/// Log the user out of their current session.
pub async fn logout(State(state): State<Arc<AppState>>, jar: CookieJar) -> impl IntoResponse {
    let Some(token) = jar.get(strings::SESSION_COOKIE_NAME) else {
        return (jar, StatusCode::OK);
    };
    let _ = helpers::delete_session(&state, token.value_trimmed().to_string()).await;
    (jar.remove(strings::SESSION_COOKIE_NAME), StatusCode::OK)
}
