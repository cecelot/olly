use crate::server::state::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::Redirect,
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use base64::Engine;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(credentials): Json<Credentials>,
) -> Result<(CookieJar, Redirect), Response<Body>> {
    let Credentials { username, password } = credentials;
    let user = match crate::server::helpers::get_user(&state, &username).await {
        Ok(user) => user,
        Err((message, code)) => return Err(response(message, code)),
    };
    if let Err((message, code)) =
        crate::server::helpers::ensure_valid_password(&user.password, &password)
    {
        return Err(response(message, code));
    }
    // Generate a random key to use as the session token.
    let key = {
        let mut dst = [0; 32];
        // ThreadRng satisfies the CryptoRng trait, so
        // it should be cryptographically secure. TODO:
        // look into a more secure key generation method.
        rand::thread_rng().fill_bytes(&mut dst);
        base64::prelude::BASE64_STANDARD.encode(dst)
    };
    let token = match crate::server::helpers::create_session(&state, &user, key).await {
        Ok(token) => token,
        Err((message, code)) => return Err(response(message, code)),
    };
    Ok((
        jar.add(Cookie::new(
            crate::server::errors::SESSION_COOKIE_NAME,
            token.clone(),
        )),
        Redirect::to("/@me"),
    ))
}

fn response(message: String, code: StatusCode) -> Response<Body> {
    let body = json!({
        "message": message,
        "code": code.as_u16(),
    });
    let body = serde_json::to_string(&body).unwrap();
    Response::builder()
        .status(code)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}
