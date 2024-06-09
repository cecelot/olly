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

pub async fn send(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    user: User,
) -> Result<impl IntoResponse, Response> {
    let other = helpers::get_user(&state, &username, true).await?;
    let request = FriendRequestAM {
        sender: ActiveValue::Set(user.id),
        recipient: ActiveValue::Set(other.id),
    };
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

pub async fn reply(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((username, outcome)): Path<(String, String)>,
) -> Result<impl IntoResponse, Response> {
    let other = helpers::get_user(&state, &username, true).await?;
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
    FriendRequest::delete(request.into_active_model())
        .exec(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
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

pub async fn cancel(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let other = helpers::get_user(&state, &username, true).await?;
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
    FriendRequest::delete(request.into_active_model())
        .exec(state.database.as_ref())
        .await
        .map_err(|e| StringError(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(super::Response::new(json!({}), StatusCode::OK))
}

#[cfg(test)]
mod tests {
    use crate::server::handlers::Response;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn send() {
        let database = sea_orm::Database::connect("postgres://olly:password@localhost:5432/olly")
            .await
            .unwrap();
        let url = test_utils::init(crate::server::app(database)).await;
        let client = test_utils::Client::new();
        let register = vec![
            serde_json::json!({
                "username": "test3",
                "password": "test3",
            }),
            serde_json::json!({
                "username": "test4",
                "password": "test4",
            }),
        ];
        for body in &register {
            client
                .post::<_, test_utils::Map>(&url, "/register", body)
                .await;
        }
        client
            .post::<_, test_utils::Map>(&url, "/login", &register[0])
            .await;
        let resp: Response<test_utils::Map> = client
            .post(&url, "/users/test4/friend", serde_json::json!({}))
            .await;
        assert_eq!(resp.code, StatusCode::CREATED);
    }
}
