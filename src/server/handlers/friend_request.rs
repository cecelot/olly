use super::StringError;
use crate::server::{
    entities::{
        friend::{ActiveModel, Column as FriendColumn},
        friend_request::{ActiveModel as FriendRequestAM, Column as FriendRequestColumn},
        prelude::{Friend, FriendRequest},
    },
    extractors::User,
    helpers,
    state::AppState,
    strings,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde_json::json;
use std::sync::Arc;

/// Send a friend request to the specified user.
pub async fn send(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response> {
    // Fetch the user object associated with the recipient username to ensure that it exists.
    let other = helpers::get_user(&state, &username, true).await?;
    // A user can't become friends with themself.
    if user.id == other.id {
        return Err(
            StringError(strings::FRIEND_SELF.to_string(), StatusCode::BAD_REQUEST).into_response(),
        );
    }
    // Check if the two users are already friends.
    let friend = Friend::find()
        .filter(
            FriendColumn::A
                .eq(user.id)
                .and(FriendColumn::B.eq(other.id))
                .or(FriendColumn::A
                    .eq(other.id)
                    .and(FriendColumn::B.eq(user.id))),
        )
        .one(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    if friend.is_some() {
        return Err(StringError(
            strings::ALREADY_FRIENDS.to_string(),
            StatusCode::BAD_REQUEST,
        )
        .into_response());
    }
    // Fetch a friend request record associated with the sender and recipient to see if one already exists.
    let request = FriendRequest::find()
        .filter(
            FriendRequestColumn::Sender
                .eq(user.id)
                .or(FriendRequestColumn::Recipient.eq(user.id)),
        )
        .filter(
            FriendRequestColumn::Recipient
                .eq(other.id)
                .or(FriendRequestColumn::Sender.eq(other.id)),
        )
        .one(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    // Disallow friend requests between two users (no matter who initiated it) if one already exists.
    if request.is_some() {
        return Err(StringError(
            strings::FRIEND_REQUEST_ALREADY_SENT.to_string(),
            StatusCode::CONFLICT,
        )
        .into_response());
    }
    let request = FriendRequestAM {
        sender: ActiveValue::Set(user.id),
        recipient: ActiveValue::Set(other.id),
    };
    // Insert the friend request record into the database.
    let model = FriendRequest::insert(request)
        .exec(state.database.as_ref())
        .await;
    let model = model.map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(super::Response::new(
        json!({ "id": model.last_insert_id}),
        StatusCode::CREATED,
    ))
}

/// Reply to a friend request with the specified outcome.
/// `outcome` must be either "accept" or "decline".
pub async fn reply(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((username, outcome)): Path<(String, String)>,
) -> Result<impl IntoResponse, Response> {
    // Fetch the user object associated with the recipient username to ensure that it exists.
    let other = helpers::get_user(&state, &username, true).await?;
    // Fetch the friend request record associated with the sender and recipient.
    let Some(request) = FriendRequest::find()
        .filter(FriendRequestColumn::Recipient.eq(user.id))
        .filter(FriendRequestColumn::Sender.eq(other.id))
        .one(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?
    else {
        return Err(StringError(
            strings::FRIEND_REQUEST_NOT_FOUND.to_string(),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    };
    // Delete the friend request record from the database. This is always done regardless of outcome,
    // so we do it first to prevent code duplication.
    FriendRequest::delete(request.into_active_model())
        .exec(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    // If the user accepted the friend request, insert a new friend record into the database.
    if outcome == "accept" {
        let friend = ActiveModel {
            a: ActiveValue::Set(user.id),
            b: ActiveValue::Set(other.id),
        };
        Friend::insert(friend)
            .exec(state.database.as_ref())
            .await
            .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    }
    Ok(super::Response::new(json!({}), StatusCode::OK))
}

/// Cancel an outgoing friend request sent to the specified user.
pub async fn cancel(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, Response> {
    // Fetch the user object associated with the recipient username to ensure that it exists.
    let other = helpers::get_user(&state, &username, true).await?;
    // Fetch the friend request record associated with the sender and recipient.
    let Some(request) = FriendRequest::find()
        .filter(FriendRequestColumn::Sender.eq(user.id))
        .filter(FriendRequestColumn::Recipient.eq(other.id))
        .one(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?
    else {
        return Err(StringError(
            strings::FRIEND_REQUEST_NOT_FOUND.to_string(),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    };
    // Delete the friend request record from the database.
    FriendRequest::delete(request.into_active_model())
        .exec(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(super::Response::new(json!({}), StatusCode::OK))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::server::{self, handlers::Response};
    use axum::http::StatusCode;
    use test_utils::{function, Client};

    struct SentRequest {
        sender: String,
        recipient: String,
    }

    async fn send_friend_request(prefix: &str, url: &str) -> SentRequest {
        let sender = format!("{prefix}::1");
        let recipient = format!("{prefix}::2");
        let client = Client::authenticated(&[&sender, &recipient], url, true).await;
        let resp: Response<test_utils::Map> = client
            .post(
                url,
                &format!("/users/{recipient}/friend"),
                serde_json::json!({}),
            )
            .await;
        assert_eq!(resp.code, StatusCode::CREATED);
        SentRequest { sender, recipient }
    }

    #[tokio::test]
    async fn send() {
        let database = sea_orm::Database::connect(server::TEST_DATABASE_URI)
            .await
            .unwrap();
        let redis = redis::Client::open(server::TEST_REDIS_URI).unwrap();
        let state = Arc::new(server::AppState::new(database, redis));
        let url = test_utils::init(crate::server::app(state)).await;
        send_friend_request(&function!(), &url).await;
    }

    #[tokio::test]
    async fn accept() {
        let database = sea_orm::Database::connect(server::TEST_DATABASE_URI)
            .await
            .unwrap();
        let redis = redis::Client::open(server::TEST_REDIS_URI).unwrap();
        let state = Arc::new(server::AppState::new(database, redis));
        let url = test_utils::init(crate::server::app(state)).await;
        let SentRequest { sender, recipient } = send_friend_request(&function!(), &url).await;
        let client = Client::authenticated(&[&recipient], &url, false).await;
        let resp: Response<test_utils::Map> = client
            .post(
                &url,
                &format!("/@me/friends/{sender}/accept"),
                serde_json::json!({}),
            )
            .await;
        assert_eq!(resp.code, StatusCode::OK);
    }

    #[tokio::test]
    async fn reject() {
        let database = sea_orm::Database::connect(server::TEST_DATABASE_URI)
            .await
            .unwrap();
        let redis = redis::Client::open(server::TEST_REDIS_URI).unwrap();
        let state = Arc::new(server::AppState::new(database, redis));
        let url = test_utils::init(crate::server::app(state)).await;
        let SentRequest { sender, recipient } = send_friend_request(&function!(), &url).await;
        let client = Client::authenticated(&[&recipient], &url, false).await;
        let resp: Response<test_utils::Map> = client
            .post(
                &url,
                &format!("/@me/friends/{sender}/reject"),
                serde_json::json!({}),
            )
            .await;
        assert_eq!(resp.code, StatusCode::OK);
    }
}
