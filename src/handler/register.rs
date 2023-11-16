use crate::{
    db::user,
    error::{AppError, AppErrorType},
    form::User,
    handler::{get_client, log_error},
    password::hash,
    AppState, Result,
};
use axum::{extract::Extension, Json};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Value};
use std::sync::Arc;

static UN_VALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]{4,16}$").unwrap());
static PW_VALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{8,20}$").unwrap());

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<User>,
) -> Result<Json<Value>> {
    let handler_name = "/register";
    if !UN_VALID.is_match(&frm.username) || !PW_VALID.is_match(&frm.password) {
        return Err(AppError::bad_register());
    }
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    match user::find(&client, &frm.username).await {
        Ok(_) => return Err(AppError::multi_register()),
        Err(err) => {
            if err.types != AppErrorType::NotFound {
                return Err(err);
            }
        }
    }
    let password = hash(&frm.password)?;
    user::create(&client, &frm.username, &password)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(json!({})))
}
