use crate::{db::topic, error::AppError, form::Topic, handler::{auth::protect, get_client, get_conn, log_error}, rds::del_session, session::get_session_id, AppState, Result, session};
use axum::{
    extract::Path,
    headers::HeaderMap,
    routing::{get, put},
    Extension, Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// Backend router
pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/logout", get(logout))
        .route("/topic", get(list).post(add))
        .route("/topic/:id", put(edit).delete(del))
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    let handler_name = "Backend/index";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(json!({})))
}

pub async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    let handler_name = "Backend/logout";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    let session_id = protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    del_session(&mut conn, &session_id)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(json!({})))
}

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    let handler_name = "Backend/list";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let topics = topic::list_all(&client, user)
        .await
        .map_err(log_error(handler_name))?;
    if topics.is_empty() {
        return Ok(Json(json!({})));
    }
    let mut ids: Vec<i64> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    for topic in topics {
        ids.push(topic.id);
        titles.push(topic.title);
    }
    Ok(Json(json!({"ids": ids, "titles": titles})))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
    headers: HeaderMap,
    Json(frm): Json<Topic>,
) -> Result<Json<Value>> {
    let handler_name = "Backend/add";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    topic::create(&client, &frm, user)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(json!({})))
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path((user, id)): Path<(String, i64)>,
    headers: HeaderMap,
    Json(frm): Json<Topic>,
) -> Result<Json<Value>> {
    let handler_name = "Backend/edit";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    topic::update(&client, &frm, id)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(json!({})))
}

pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path((user, id)): Path<(String, i64)>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    let handler_name = "Backend/del";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let res = topic::del(&client, id)
        .await
        .map_err(log_error(handler_name))?;
    if !res {
        return Err(AppError::duplication());
    }
    Ok(Json(json!({})))
}
