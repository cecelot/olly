use super::*;

pub async fn create_session(
    state: &AppState,
    user: &member::Model,
    key: String,
) -> Result<String, (String, StatusCode)> {
    match entities::prelude::Session::insert(session::ActiveModel {
        id: ActiveValue::set(user.id),
        key: ActiveValue::set(key.clone()),
    })
    .on_conflict(
        OnConflict::column(session::Column::Id)
            .update_column(session::Column::Key)
            .to_owned(),
    )
    .exec(state.database.as_ref())
    .await
    {
        Ok(_) => Ok(key),
        Err(e) => Err((e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub async fn delete_session(state: &AppState, token: String) -> Result<(), (String, StatusCode)> {
    match entities::prelude::Session::delete_many()
        .filter(entities::session::Column::Key.eq(token))
        .exec(state.database.as_ref())
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err((e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub fn ensure_valid_password(actual: &str, provided: &str) -> Result<(), (String, StatusCode)> {
    let hashed = PasswordHash::new(actual).map_err(|_| {
        (
            errors::INVALID_PASSWORD_FORMAT.to_string(),
            StatusCode::BAD_REQUEST,
        )
    })?;
    if Argon2::default()
        .verify_password(provided.as_bytes(), &hashed)
        .is_err()
    {
        Err((errors::INVALID_PASSWORD.to_string(), StatusCode::FORBIDDEN))
    } else {
        Ok(())
    }
}

pub async fn get_user(
    state: &AppState,
    username: &String,
) -> Result<member::Model, (String, StatusCode)> {
    match entities::prelude::Member::find()
        .filter(member::Column::Username.eq(username))
        .one(state.database.as_ref())
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err((errors::INVALID_USERNAME.to_string(), StatusCode::NOT_FOUND)),
        Err(e) => Err((e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
