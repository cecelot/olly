use crate::server::state::AppState;
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response as AxumResponse,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod entities;
mod errors;
mod handlers;
mod packet;
mod state;

// is this poor design? maybe the tests that require these
// should be in this module rather than in the top-level
// tests module.
pub use entities::{game, member, session};
pub use handlers::Response as HttpResponse;
pub use packet::Event as SocketEvent;

pub fn app(database: DatabaseConnection) -> Router {
    let state = Arc::new(AppState::new(database));
    Router::new()
        .route("/live", get(handler).with_state(Arc::clone(&state)))
        .route("/register", post(handlers::register).with_state(state))
        .fallback(handlers::fallback)
}

async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> AxumResponse {
    ws.on_upgrade(|socket| handlers::callback(socket, state))
}
