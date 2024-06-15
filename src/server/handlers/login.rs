use crate::server::{helpers, state::AppState, strings};
use axum::{
    body::Body,
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use base64::Engine;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

/// Authenticate the user with the specified credentials.
pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, Response<Body>> {
    let Credentials { username, password } = credentials;
    let user = helpers::get_user(&state, &username, true).await?;
    helpers::ensure_valid_password(&user.password, &password)?;
    // Generate a random key to use as the session token.
    let key = {
        let mut dst = [0; 32];
        // ThreadRng satisfies the CryptoRng trait, so
        // it should be cryptographically secure. TODO:
        // look into a more secure key generation method.
        rand::thread_rng().fill_bytes(&mut dst);
        base64::prelude::BASE64_STANDARD.encode(dst)
    };
    let token = helpers::create_session(&state, &user, key).await?;
    Ok((
        jar.add(Cookie::new(strings::SESSION_COOKIE_NAME, token.clone())),
        Redirect::to("/@me"),
    ))
}

#[cfg(test)]
mod tests {
    use crate::server;
    use test_utils::{function, Client};

    #[tokio::test]
    async fn login() {
        let database = sea_orm::Database::connect(server::INSECURE_DEFAULT_DATABASE_URL)
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        let client = Client::authenticated(&[&function!()], &url, true).await;
        let res: serde_json::Value = client.get(&url, "/@me").await;
        assert_eq!(&res["code"], &200);
    }
}
