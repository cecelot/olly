use axum::{body::Body, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    pub fn new(message: S, code: StatusCode) -> (StatusCode, Json<Self>) {
        (
            code,
            Json(Self {
                message,
                code: u16::from(code),
            }),
        )
    }

    pub fn raw(message: S, code: StatusCode) -> axum::response::Response<Body> {
        let body = serde_json::to_string(&json!({
            "message": message,
            "code": code.as_u16(),
        }))
        .unwrap();
        axum::response::Response::builder()
            .status(code)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

pub struct StringError(pub String, pub StatusCode);

impl IntoResponse for StringError {
    fn into_response(self) -> axum::response::Response {
        Response::new(self.0, self.1).into_response()
    }
}

impl From<StringError> for axum::response::Response {
    fn from(e: StringError) -> Self {
        e.into_response()
    }
}

pub async fn fallback() -> impl IntoResponse {
    Response::new("not found", StatusCode::NOT_FOUND)
}
