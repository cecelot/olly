use axum::Router;
use serde::Serialize;
use serde_json::json;
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn post<S: Serialize>(&self, url: &str, endpoint: &str, body: S) -> String {
        let res = self
            .inner
            .post(&format!("{url}{endpoint}"))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
            .unwrap();
        res.text().await.unwrap()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn init(app: Router) -> String {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(axum::serve(listener, app).into_future());
    let url = format!("http://{}", addr);
    let client = Client::new();
    let body = json!({"username": "alaidriel", "password": "meow"});
    client.post(&url, "/register", body).await;
    url
}
