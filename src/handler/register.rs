use crate::{
    db::user,
    error::AppError,
    form::User,
    handler::{get_client, log_error},
    password::hash,
    AppState, Result,
};
use axum::{extract::Extension, Json};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;
use serde_json::{Value, json};

static UN_VALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]{4,16}$").unwrap());
static PW_VALID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{8,20}$").unwrap());

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<User>,
) -> Result<Json<Value>> {
    let handler_name = "/register";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    if !UN_VALID.is_match(&frm.username) || !PW_VALID.is_match(&frm.password) {
        return Err(AppError::bad_register());
    }
    let password = hash(&frm.password)?;
    let n = user::create(&client, &frm.username, &password).await.map_err(log_error(handler_name))?;
    if n < 1 {
        return Err(AppError::multi_register())
    }
    Ok(Json(json!({})))
}
