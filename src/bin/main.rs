use std::sync::Arc;

use olly::server::{
    app, restore_active_games, AppState, DEFAULT_REDIS_URL, INSECURE_DEFAULT_DATABASE_URL,
};
use sea_orm::Database;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Error)
        .init();
    // Get the database URL from the environment, or use the insecure default.
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or(String::from(INSECURE_DEFAULT_DATABASE_URL));
    let redis_url = std::env::var("REDIS_URL").unwrap_or(String::from(DEFAULT_REDIS_URL));
    // Connect to the database and bind a TCP listener to :3000.
    let database = Database::connect(database_url).await.unwrap();
    let redis = redis::Client::open(redis_url).unwrap();
    // Ensure the connection to the database is established.
    let _ = redis.get_connection().unwrap();
    let state = Arc::new(AppState::new(database, redis));
    // Restore any active games to the cache.
    restore_active_games(&state).await?;
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // Serve the app on the port specified above.
    axum::serve(listener, app(state)).await.unwrap();
    Ok(())
}
