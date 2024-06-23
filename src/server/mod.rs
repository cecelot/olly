use crate::Game;
use argon2::PasswordHash;
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Router,
};
use entities::game::Column;
use handlers::StringError;
use redis::Commands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

pub use state::AppState;

mod entities;
mod extractors;
mod handlers;
mod helpers;
mod packet;
mod state;
mod strings;

/// This is highly insecure, but useful for development/testing.
pub const INSECURE_DEFAULT_DATABASE_URL: &str = "postgres://olly:password@db:5432/olly";
pub const DEFAULT_REDIS_URL: &str = "redis://cache";

pub fn app(state: Arc<AppState>) -> Router {
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
            "/@me",
            patch(handlers::update_me).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/games",
            get(handlers::active_games).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/games/pending",
            get(handlers::pending_games).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/games/:id/cancel",
            delete(handlers::cancel_invite).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/games/:id/accept",
            post(handlers::accept_game).with_state(Arc::clone(&state)),
        )
        .route(
            "/@me/games/:id/decline",
            delete(handlers::decline_game).with_state(Arc::clone(&state)),
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

/// Create a new game with the specified host and guest.
/// # Panics
/// Panics if the mutex is poisoned.
pub fn create_in_memory_game(state: &Arc<AppState>, gid: Uuid) {
    // Create a new game object and broadcast channel for notifications to websocket
    // subscribers.
    let mut conn = state.redis.get_connection().unwrap();
    let game = if let Ok(cached) = conn.get::<String, String>(format!("game:{gid}")) {
        let game: Game = serde_json::from_str(&cached).unwrap();
        log::info!("Restoring {gid:?} from cache: raw {cached}");
        game
    } else {
        Game::new()
    };
    let (tx, _) = broadcast::channel(16);
    // Insert the game object and broadcast channel into the global state.
    let mut games = state.games.lock().expect("mutex was poisoned");
    let mut rooms = state.rooms.lock().expect("mutex was poisoned");
    games.insert(gid, game);
    rooms.insert(gid, tx);
}

/// Restore any active games to the cache.
/// # Errors
/// If an error occurs while querying the database, it will be returned as a string.
pub async fn restore_active_games(state: &Arc<AppState>) -> Result<(), String> {
    let games = entities::game::Entity::find()
        .filter(Column::Pending.eq(false))
        .all(state.database.as_ref())
        .await
        .map_err(|e| e.to_string())?;
    for game in &games {
        create_in_memory_game(state, game.id);
    }
    Ok(())
}

/// Validates a username according to the following rules:
/// - At least three characters long
/// # Errors
/// The username does not meet the above criteria.
pub fn validate_username(username: &str) -> Result<(), StringError> {
    // Ensure that the username is at least 3 characters long. Totally arbitrary.
    if username.len() < 3 {
        return Err(StringError(
            strings::USERNAME_TOO_SHORT.into(),
            StatusCode::BAD_REQUEST,
        ));
    }
    Ok(())
}

/// Validates a password according to the following rules:
/// - At least 8 characters long
/// - Contains at least one alphabetic character
/// - Contains at least one numeric character
/// # Errors
/// The password does not meet the above criteria.
pub fn validate_password(password: &str) -> Result<(), StringError> {
    // Make sure that the password is at least 8 characters long. Protects against grossly insecure
    // passwords while not being too annoying for uncaring users.
    if password.len() < 8 {
        return Err(StringError(
            strings::PASSWORD_TOO_SHORT.into(),
            StatusCode::BAD_REQUEST,
        ));
    }
    // Ensure that the password contains at least one alphabetic character and one number.
    // An additional check to ensure the password isn't horribly insecure.
    match password {
        _ if !password.contains(|c: char| c.is_alphabetic()) => {
            return Err(StringError(
                strings::PASSWORD_NO_ALPHA.into(),
                StatusCode::BAD_REQUEST,
            ));
        }
        _ if !password.contains(|c: char| c.is_numeric()) => {
            return Err(StringError(
                strings::PASSWORD_NO_NUMERIC.into(),
                StatusCode::BAD_REQUEST,
            ));
        }
        _ => {}
    }
    Ok(())
}
