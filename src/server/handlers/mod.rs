use crate::server::Response;
use axum::{http::StatusCode, Json};

pub mod live;
pub mod register;

pub use live::callback;
pub use register::register;

pub async fn fallback() -> Json<Response> {
    Response::error("not found", StatusCode::NOT_FOUND).into()
}
