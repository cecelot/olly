use super::StringError;
use crate::server::{
    entities::{
        friend::ActiveModel,
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
use sea_orm::{
    ActiveValue, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter, RuntimeErr,
};
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
    let request = FriendRequestAM {
        sender: ActiveValue::Set(user.id),
        recipient: ActiveValue::Set(other.id),
    };
    // Insert the friend request record into the database.
    let model = FriendRequest::insert(request)
        .exec(state.database.as_ref())
        .await;
    let model = model.map_err(|e| match e {
        DbErr::Exec(RuntimeErr::SqlxError(e))
            if e.as_database_error()
                .is_some_and(|e| e.code().is_some_and(|code| code == "23505")) =>
        {
            StringError(
                strings::FRIEND_REQUEST_ALREADY_SENT.into(),
                StatusCode::CONFLICT,
            )
        }
        _ => StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
    })?;
    Ok(super::Response::new(
        json!({ "id": model.last_insert_id}),
        StatusCode::CREATED,
    ))
}

/// Reply to a friend request with the specified outcome.
/// `outcome` must be either "accept" or "reject".
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
        let database = sea_orm::Database::connect(server::INSECURE_DEFAULT_DATABASE_URL)
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        send_friend_request(&function!(), &url).await;
    }

    #[tokio::test]
    async fn accept() {
        let database = sea_orm::Database::connect(server::INSECURE_DEFAULT_DATABASE_URL)
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
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
        let database = sea_orm::Database::connect(server::INSECURE_DEFAULT_DATABASE_URL)
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
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
