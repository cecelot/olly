use self::state::AppState;
use argon2::PasswordHash;
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod entities;
mod extractors;
mod handlers;
mod helpers;
mod packet;
mod state;
mod strings;

/// This is highly insecure, but useful for development/testing.
pub const INSECURE_DEFAULT_DATABASE_URL: &str = "postgres://olly:password@0.0.0.0:5432/olly";

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
        .route(
            "/game",
            post(handlers::create).with_state(Arc::clone(&state)),
        )
        .route(
            "/game/:id",
            get(handlers::game).with_state(Arc::clone(&state)),
        )
        .route(
            "/users/:id/friend",
            post(handlers::friend_request::send).with_state(Arc::clone(&state)),
        )
        .route("/@me", get(handlers::me).with_state(Arc::clone(&state)))
        .route(
            "/@me/games",
            get(handlers::games).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/friends",
            get(handlers::friends).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/friends/:id",
            delete(handlers::remove_friend).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/friends/incoming",
            get(handlers::incoming).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/friends/outgoing",
            get(handlers::outgoing).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/friends/outgoing/:id",
            delete(handlers::friend_request::cancel).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/friends/:id/:outcome",
            post(handlers::friend_request::reply).with_state(Arc::clone(&state)),
        )
        .route("/companion", post(handlers::companion).with_state(state))
        .fallback(handlers::fallback)
        // TODO: Use a proper CORS policy.
        .layer(CorsLayer::very_permissive())
}

async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handlers::callback(socket, state))
}
