use std::sync::Arc;

use crate::server::{
    entities::{
        friend::Column as FriendColumn,
        friend_request::Column as FriendRequestColumn,
        game::Column as GameColumn,
        prelude::{Friend, FriendRequest, Game},
    },
    extractors::User,
    handlers::StringError,
    helpers,
    state::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

pub async fn me(user: User) -> Result<impl IntoResponse, Response> {
    Ok(user)
}

pub async fn games(
    State(state): State<Arc<AppState>>,
    user: User,
) -> Result<impl IntoResponse, Response> {
    let games = Game::find()
        .filter(
            GameColumn::Host
                .eq(user.id.to_string())
                .or(GameColumn::Guest.eq(user.id.to_string())),
        )
        .all(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let mut resp = vec![];
    for g in &games {
        let id = if user.id.to_string() == g.host {
            &g.guest
        } else {
            &g.host
        };
        let opponent = helpers::get_user(&state, id, false).await?;
        resp.push(json!({
            "id": g.id,
            "opponent": opponent.username
        }));
    }
    Ok(super::Response::new(resp, StatusCode::OK))
}

pub async fn incoming(
    State(state): State<Arc<AppState>>,
    user: User,
) -> Result<impl IntoResponse, Response> {
    let frs = FriendRequest::find()
        .filter(FriendRequestColumn::Recipient.eq(user.id))
        .all(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let mut incoming = vec![];
    for fr in &frs {
        let sender = helpers::get_user(&state, &fr.sender.to_string(), false).await?;
        incoming.push(json!({
            "sender": sender.username,
        }));
    }
    Ok(super::Response::new(incoming, StatusCode::OK))
}

pub async fn outgoing(
    State(state): State<Arc<AppState>>,
    user: User,
) -> Result<impl IntoResponse, Response> {
    let frs = FriendRequest::find()
        .filter(FriendRequestColumn::Sender.eq(user.id))
        .all(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let mut outgoing = vec![];
    for fr in &frs {
        let recipient = helpers::get_user(&state, &fr.recipient.to_string(), false).await?;
        outgoing.push(json!({
            "recipient": recipient.username,
        }));
    }
    Ok(super::Response::new(outgoing, StatusCode::OK))
}

pub async fn friends(
    State(state): State<Arc<AppState>>,
    user: User,
) -> Result<impl IntoResponse, Response> {
    let friends = Friend::find()
        .filter(FriendColumn::A.eq(user.id).or(FriendColumn::B.eq(user.id)))
        .all(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let mut f = vec![];
    for friend in &friends {
        let id = if friend.a == user.id {
            &friend.b
        } else {
            &friend.a
        };
        let friend = helpers::get_user(&state, &id.to_string(), false).await?;
        f.push(json!({
            "username": friend.username,
        }));
    }
    Ok(super::Response::new(f, StatusCode::OK))
}

#[cfg(test)]
mod tests {
    use crate::server::handlers::Response;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Friend {
        id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Incoming {
        sender: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Outgoing {
        recipient: String,
    }

    #[tokio::test]
    #[ignore = "race - for experimental purposes, not a formal test yet"]
    async fn friends() {
        let database = sea_orm::Database::connect("postgres://olly:password@localhost:5432/olly")
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        let client = test_utils::Client::new();
        let credentials = serde_json::json!({
            "username": "test4",
            "password": "test4"
        });
        client
            .post::<_, test_utils::Map>(&url, "/login", credentials)
            .await;
        let friends: Response<Vec<Friend>> = client.get(&url, "/@me/friends").await;
        let outgoing: Response<Vec<Outgoing>> = client.get(&url, "/@me/friends/outgoing").await;
        let incoming: Response<Vec<Incoming>> = client.get(&url, "/@me/friends/incoming").await;
        println!("{friends:?}");
        println!("{outgoing:?}");
        println!("{incoming:?}");
    }
}
