use crate::{
    db::user,
    error::AppError,
    form::User,
    handler::{get_client, get_conn, log_error},
    password::hash,
    rds::{add_user, is_user},
    AppState, Result,
};
use axum::{extract::Extension, Json};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;

static UN_VALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]{4,16}$").unwrap());
static PW_VALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{8,20}$").unwrap());

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<User>,
) -> Result<()> {
    let handler_name = "Register";
    if !UN_VALID.is_match(&frm.username) || !PW_VALID.is_match(&frm.password) {
        return Err(log_error(handler_name)(AppError::bad_request(
            "Some field is wrong!",
        )));
    }
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    let res = is_user(&mut conn, &frm.username)
        .await
        .map_err(log_error(handler_name))?;
    if res {
        return Err(log_error(handler_name)(AppError::bad_request(
            "Username existed!",
        )));
    }
    let password = hash(&frm.password).map_err(log_error(handler_name))?;
    user::create(&client, &frm.username, &password)
        .await
        .map_err(log_error(handler_name))?;
    add_user(&mut conn, &frm.username)
        .await
        .map_err(log_error(handler_name))?;
    Ok(())
}
