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
    use crate::Game;
    use reqwest::Client;
    use sea_orm::Database;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::{
        future::IntoFuture,
        net::{Ipv4Addr, SocketAddr},
    };
    use tokio::net::TcpListener;

    async fn init() -> String {
        let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
            .await
            .unwrap();
        let addr = listener.local_addr().unwrap();
        let database =
            Database::connect("postgres://othello-server:password@0.0.0.0:5432/othello-server")
                .await
                .unwrap();
        tokio::spawn(axum::serve(listener, crate::server::app(database)).into_future());
        let body = json!({"username": "alaidriel", "password": "meow"});
        let body = serde_json::to_string(&body).unwrap();
        let _ = reqwest::Client::new()
            .post(format!("http://{addr}/register"))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await;
        format!("http://{}", addr)
    }

    async fn post<S: Serialize>(client: &Client, url: &str, endpoint: &str, body: S) -> String {
        let res = client
            .post(&format!("{url}{endpoint}"))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
            .unwrap();
        res.text().await.unwrap()
    }

    #[tokio::test]
    async fn new() {
        let url = init().await;
        let game = Game::new();
        let client = Client::builder().cookie_store(true).build().unwrap();
        let credentials = json!({
            "username": "alaidriel",
            "password": "meow"
        });
        post(&client, &url, "/login", credentials).await;
        let choice = post(&client, &url, "/companion", &game).await;
        #[derive(Deserialize)]
        struct Choice {
            message: (usize, usize),
            code: usize,
        }
        let choice: Choice = serde_json::from_str(&choice).unwrap();
        assert_eq!(choice.code, 200);
        assert_eq!(choice.message, (5, 4));
    }
}
