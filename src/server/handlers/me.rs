use crate::server::extractors::User;
use axum::response::{IntoResponse, Response};

pub async fn me(user: User) -> Result<impl IntoResponse, Response> {
    Ok(user)
}
