use crate::server::{handlers::StringError, helpers, state::AppState, strings, HttpResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use std::sync::Arc;

pub async fn me(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, Response> {
    let sid = jar
        .get(crate::server::strings::SESSION_COOKIE_NAME)
        .ok_or(StringError(
            strings::INVALID_TOKEN.into(),
            StatusCode::UNAUTHORIZED,
        ))?
        .value_trimmed();
    let session = helpers::get_session(&state, sid).await?;
    let user = helpers::get_user(&state, &session, false).await?;
    Ok(HttpResponse::new(
        json!({
            "id": user.id,
            "username": user.username,
        }),
        StatusCode::OK,
    ))
}
