use axum::Router;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

#[macro_export]
/// This macro is used to get the name of the function that calls it.
/// It is used for unique identifiers for tests that require authentication
/// because a single test cannot rely on another test to have run first
/// and created a user.
///
/// Adapted from: https://stackoverflow.com/a/63904992
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let mut actual = String::new();
        actual.push_str(&name[..name.len() - 3]);
        // This ensures that the password meets the minimum requirements.
        actual.push('1');
        actual
    }};
}

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

    pub async fn authenticated(credentials: &[&str], url: &str, register: bool) -> Client {
        let client = Client::new();
        let credentials: Vec<_> = credentials
            .iter()
            .map(|id| {
                serde_json::json!({
                    "username": id,
                    "password": id
                })
            })
            .collect();
        if register {
            for credential in &credentials {
                client.post::<_, Map>(url, "/register", credential).await;
            }
        }
        client.post::<_, Map>(url, "/login", &credentials[0]).await;
        client
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
    format!("http://{addr}")
}
