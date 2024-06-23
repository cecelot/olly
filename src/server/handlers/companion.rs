use crate::{
    companion::Companion,
    server::{extractors::User, handlers::Response},
    Game,
};
use axum::{http::StatusCode, response::IntoResponse, Json};

/// The default search depth for the companion. Chosen arbitrarily with the goal of providing quality moves in a reasonable amount of time.
const DEFAULT_DEPTH: usize = 6;

/// Provide the best available move for the given game state.
pub async fn companion(
    _: User, // We don't care who the user is, just that this is an authenticated request
    body: Json<Game>,
) -> Result<impl IntoResponse, axum::response::Response> {
    let mut companion = Companion::from(&body.0);
    // TODO: Allow the user to specify a custom search depth.
    let depth = DEFAULT_DEPTH;
    Ok(Response::new(companion.choice(depth), StatusCode::OK))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::server;
    use test_utils::{function, Client};

    #[derive(serde::Deserialize)]
    struct Choice {
        message: (usize, usize),
        code: usize,
    }

    #[tokio::test]
    async fn new() {
        let database = sea_orm::Database::connect(server::TEST_DATABASE_URI)
            .await
            .unwrap();
        let redis = redis::Client::open(server::TEST_REDIS_URI).unwrap();
        let state = Arc::new(server::AppState::new(database, redis));
        let url = test_utils::init(crate::server::app(state)).await;
        let game = crate::Game::new();
        let client = Client::authenticated(&[&function!()], &url, true).await;
        let choice: Choice = client.post(&url, "/companion", &game).await;
        assert_eq!(choice.code, 200);
        assert_eq!(choice.message, (5, 4));
    }
}
