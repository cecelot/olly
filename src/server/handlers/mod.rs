use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

mod companion;
mod create;
pub mod friend_request;
mod game;
mod live;
mod login;
mod logout;
mod me;
mod register;

pub use companion::companion;
pub use create::create;
pub use game::{accept as accept_game, cancel as cancel_invite, decline as decline_game, game};
pub use live::callback;
pub use login::login;
pub use logout::logout;
pub use me::{
    active_games, friends, incoming, me, outgoing, pending_games, remove_friend,
    update as update_me,
};
pub use register::register;

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug)]
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
