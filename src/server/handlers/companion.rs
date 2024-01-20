use crate::{
    companion::Companion,
    server::{extractors::User, handlers::Response},
    Game,
};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn companion(
    _: User, // We don't care who the user is, just that this is an authenticated request
    body: Json<Game>,
) -> Result<impl IntoResponse, axum::response::Response> {
    let mut companion = Companion::from(&body.0);
    Ok(Response::new(companion.choice(6), StatusCode::OK))
}

#[cfg(test)]
mod tests {
    #[derive(serde::Deserialize)]
    struct Choice {
        message: (usize, usize),
        code: usize,
    }

    #[tokio::test]
    async fn new() {
        let database = sea_orm::Database::connect(
            "postgres://othello-server:password@0.0.0.0:5432/othello-server",
        )
        .await
        .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        let client = test_utils::Client::new();
        let game = crate::Game::new();
        let credentials = serde_json::json!({
            "username": "test",
            "password": "test"
        });
        client
            .post::<_, test_utils::Map>(&url, "/login", credentials)
            .await;
        let choice: Choice = client.post(&url, "/companion", &game).await;
        assert_eq!(choice.code, 200);
        assert_eq!(choice.message, (5, 4));
    }
}
