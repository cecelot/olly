use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

pub mod live;
pub mod register;

pub use live::callback;
pub use register::register;

#[derive(Serialize, Deserialize)]
pub struct Response {
    message: String,
    code: u16,
}

impl Response {
    pub fn new<S: Serialize>(message: S, code: StatusCode) -> Self {
        Self {
            message: serde_json::to_string(&message).unwrap(),
            code: u16::from(code),
        }
    }
}

pub async fn fallback() -> Json<Response> {
    Response::new("not found", StatusCode::NOT_FOUND).into()
}
