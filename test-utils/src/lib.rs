use axum::Router;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

pub type Map = serde_json::Map<String, serde_json::Value>;

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

    pub async fn get<D: DeserializeOwned>(&self, url: &str, endpoint: &str) -> D {
        let res = self
            .inner
            .get(&format!("{url}{endpoint}"))
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        serde_json::from_str(&text).unwrap()
    }

    pub async fn post<S: Serialize, D: DeserializeOwned>(
        &self,
        url: &str,
        endpoint: &str,
        body: S,
    ) -> D {
        let res = self
            .inner
            .post(&format!("{url}{endpoint}"))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        serde_json::from_str(&text).unwrap()
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
    let body = json!({"username": "test", "password": "test"});
    client.post::<_, Map>(&url, "/register", body).await;
    url
}
