use crate::{
    db::topic,
    error::AppError,
    handler::{get_client, log_error},
    AppState, Result,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Args {
    pub level: i32,
}

pub async fn topic(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Query(args): Query<Args>,
) -> Result<Json<Value>> {
    let handler_name = "Topic";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    if args.level == 0 {
        let mut tmp = topic::content(&client, id)
            .await
            .map_err(log_error(handler_name))?;
        if tmp.is_empty() {
            return Err(log_error(handler_name)(AppError::bad_request(
                "Data required non-existent!",
            )));
        }
        let topic = tmp.pop().unwrap();
        let html = topic.html;
        Ok(Json(json!({"html": html})))
    } else {
        let mut tmp = topic::edit_data(&client, id)
            .await
            .map_err(log_error(handler_name))?;
        if tmp.is_empty() {
            return Err(log_error(handler_name)(AppError::bad_request(
                "Data required non-existent!",
            )));
        }
        let topic = tmp.pop().unwrap();
        let (title, summary, markdown) = (topic.title, topic.summary, topic.markdown);
        Ok(Json(
            json!({"title": title, "summary": summary, "markdown": markdown}),
        ))
    }
}
