use crate::{
    db::user,
    error::AppError,
    form::User,
    handler::{get_client, get_conn, log_error},
    password,
    rds::set_session,
    session::set_session_id,
    AppState, Result,
};
use axum::{extract::Extension, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<User>,
) -> Result<Json<Value>> {
    let handler_name = "Login";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let mut user = user::find(&client, &frm.username)
        .await
        .map_err(log_error(handler_name))?;
    if user.is_empty() {
        return Err(log_error(handler_name)(AppError::bad_request(
            "Username or password error!",
        )));
    }
    let user_info = user.pop().unwrap();
    let verify =
        password::verify(&frm.password, &user_info.password).map_err(log_error(handler_name))?;
    if !verify {
        return Err(log_error(handler_name)(AppError::bad_request(
            "Username or password error!",
        )));
    }
    let session_id = Uuid::new_v4().to_string();
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    set_session(&mut conn, &session_id, &user_info.username)
        .await
        .map_err(log_error(handler_name))?;
    let cookie = set_session_id(&session_id);
    Ok(Json(json!({"cookie": cookie})))
}
