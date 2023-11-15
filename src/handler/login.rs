use crate::{
    db::user,
    error::{AppError, AppErrorType},
    form::User,
    handler::{get_client, get_conn, log_error, redirect, redirect_with_session, RedirectView},
    password,
    rds::{del_session, set_session},
    session::get_session_id,
    AppState, Result,
};
use axum::{extract::Extension, http::HeaderMap, Json};
use std::sync::Arc;
use uuid::Uuid;

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(frm): Json<User>,
) -> Result<RedirectView> {
    let handler_name = "/login";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let user_info = user::find(&client, &frm.username)
        .await
        .map_err(|err| match err.types {
            AppErrorType::NotFound => AppError::incorrect_login(),
            _ => err,
        })
        .map_err(log_error(handler_name))?;
    let verify =
        password::verify(&frm.password, &user_info.password).map_err(log_error(handler_name))?;
    if !verify {
        return Err(AppError::incorrect_login());
    }
    let session_id = Uuid::new_v4().to_string();
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    set_session(&mut conn, &session_id, &user_info.username)
        .await
        .map_err(log_error(handler_name))?;
    let url = format!("/admin/{}", &user_info.username);
    redirect_with_session(&url, Some(&session_id))
}

pub async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<RedirectView> {
    let handler_name = "/logout";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    if let Some(session_id) = get_session_id(&headers) {
        del_session(&mut conn, &session_id)
            .await
            .map_err(log_error(handler_name))?;
    }
    redirect("/login.html")
}
