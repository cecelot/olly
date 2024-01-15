use self::state::AppState;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::StatusCode,
    response::Response as AxumResponse,
    routing::{get, post},
    Router,
};
use sea_orm::{
    sea_query::OnConflict, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod entities;
mod errors;
mod handlers;
mod helpers;
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
        .route(
            "/register",
            post(handlers::register).with_state(Arc::clone(&state)),
        )
        .route(
            "/login",
            post(handlers::login).with_state(Arc::clone(&state)),
        )
        .route(
            "/logout",
            post(handlers::logout).with_state(Arc::clone(&state)),
        )
        .route("/@me", get(handlers::me).with_state(state))
        .fallback(handlers::fallback)
        // TODO: Use a prper CORS policy.
        .layer(CorsLayer::very_permissive())
}

async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> AxumResponse {
    ws.on_upgrade(|socket| handlers::callback(socket, state))
}
