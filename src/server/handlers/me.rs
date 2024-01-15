use crate::server::{
    entities::{
        prelude::{Member, Session},
        session,
    },
    errors,
    state::AppState,
    HttpResponse,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::sync::Arc;

pub async fn me(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {
    let sid = jar
        .get(crate::server::errors::SESSION_COOKIE_NAME)
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(HttpResponse::new(
                errors::INVALID_TOKEN.into(),
                StatusCode::UNAUTHORIZED,
            )),
        ))?
        .value_trimmed();
    let session = match Session::find()
        .filter(session::Column::Key.eq(sid))
        .one(state.database.as_ref())
        .await
    {
        Ok(Some(session)) => session,
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(HttpResponse::new(
                    errors::INVALID_TOKEN.into(),
                    StatusCode::UNAUTHORIZED,
                )),
            ))
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HttpResponse::new(
                    e.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            ))
        }
    };
    let user = match Member::find_by_id(session.id)
        .one(state.database.as_ref())
        .await
    {
        Ok(Some(user)) => user,
        e => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HttpResponse::new(
                    format!("{:?}", e),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            ))
        }
    };
    Ok((
        StatusCode::OK,
        Json(HttpResponse::new(
            json!({
                "id": user.id,
                "username": user.username,
            }),
            StatusCode::OK,
        )),
    ))
}
