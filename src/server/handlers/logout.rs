use crate::server::state::AppState;
use axum::{extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn logout(State(state): State<Arc<AppState>>, jar: CookieJar) -> (CookieJar, StatusCode) {
    let token = match jar.get(crate::server::errors::SESSION_COOKIE_NAME) {
        Some(token) => token,
        None => return (jar, StatusCode::OK),
    };
    let _ = crate::server::helpers::delete_session(&state, token.value_trimmed().to_string()).await;
    (
        jar.remove(crate::server::errors::SESSION_COOKIE_NAME),
        StatusCode::OK,
    )
}
