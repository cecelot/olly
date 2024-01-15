use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

mod live;
mod login;
mod logout;
mod me;
mod register;

pub use live::callback;
pub use login::login;
pub use logout::logout;
pub use me::me;
pub use register::register;

#[derive(Serialize, Deserialize)]
pub struct Response<S: Serialize> {
    message: S,
    code: u16,
}

impl<S: Serialize> Response<S> {
    pub fn new(message: S, code: StatusCode) -> Self {
        Self {
            message,
            code: u16::from(code),
        }
    }
}

pub async fn fallback() -> Json<Response<&'static str>> {
    Response::new("not found", StatusCode::NOT_FOUND).into()
}
