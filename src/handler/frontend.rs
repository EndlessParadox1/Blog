use crate::{
    db::topic,
    db::user,
    handler::{get_client, log_error},
    AppState, Result,
};
use axum::{extract::Path, routing::get, Extension, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

/// Frontend router
pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/archive/:dt", get(archive))
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Path(user): Path<String>,
) -> Result<Json<Value>> {
    let handler_name = "/frontend/index";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    user::find(&client, &user)
        .await
        .map_err(log_error(handler_name))?;
    let topics = topic::list_latest(&client, user.clone())
        .await
        .map_err(log_error(handler_name))?;
    if topics.is_empty() {
        return Ok(Json(json!({})));
    }
    let mut ids: Vec<i64> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut times: Vec<String> = Vec::new();
    let mut summaries: Vec<String> = Vec::new();
    for topic in topics {
        times.push(topic.dateline());
        ids.push(topic.id);
        titles.push(topic.title);
        summaries.push(topic.summary);
    }
    let archive = topic::archive(&client, user)
        .await
        .map_err(log_error(handler_name))?;
    let mut archs: Vec<String> = Vec::new();
    for arch in archive {
        archs.push(arch.dateline);
    }
    Ok(Json(
        json!({"ids": ids, "titles": titles, "times": times, "summaries": summaries, "archs": archs}),
    ))
}

pub async fn archive(
    Extension(state): Extension<Arc<AppState>>,
    Path((user, dt)): Path<(String, String)>,
) -> Result<Json<Value>> {
    let handler_name = "/frontend/index";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let topics = topic::list_arch(&client, user.clone(), dt)
        .await
        .map_err(log_error(handler_name))?;
    if topics.is_empty() {
        return Ok(Json(json!({})));
    }
    let mut ids: Vec<i64> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut times: Vec<String> = Vec::new();
    let mut summaries: Vec<String> = Vec::new();
    for topic in topics {
        times.push(topic.dateline());
        ids.push(topic.id);
        titles.push(topic.title);
        summaries.push(topic.summary);
    }
    Ok(Json(
        json!({"ids": ids, "titles": titles, "times": times, "summaries": summaries}),
    ))
}
