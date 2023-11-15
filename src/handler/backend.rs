use crate::{
    db::topic,
    error::AppError,
    form::Topic,
    handler::{auth::protect, get_client, get_conn, log_error},
    AppState, Result,
};
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
        .route("/topic", get(list).post(add))
        .route("/topic/:id", put(edit).delete(del))
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    let handler_name = "/backend/index";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(json!({})))
}

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>> {
    let handler_name = "/backend/list";
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
    let handler_name = "/backend/add";
    let mut conn = get_conn(&state).await.map_err(log_error(handler_name))?;
    protect(&headers, &mut conn, &user)
        .await
        .map_err(log_error(handler_name))?;
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let res = topic::create(&client, &frm, user)
        .await
        .map_err(log_error(handler_name))?;
    if !res {
        return Err(AppError::duplication());
    }
    Ok(Json(json!({})))
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path((user, id)): Path<(String, i64)>,
    headers: HeaderMap,
    Json(frm): Json<Topic>,
) -> Result<Json<Value>> {
    let handler_name = "/backend/edit";
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
    let handler_name = "/backend/del";
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
