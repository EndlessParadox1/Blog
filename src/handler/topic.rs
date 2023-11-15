use crate::{
    db::topic,
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
    let handler_name = "/topic";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    if args.level == 0 {
        let tmp = topic::content(&client, id)
            .await
            .map_err(log_error(handler_name))?;
        let html = tmp.html;
        Ok(Json(json!({"html": html})))
    } else {
        let tmp = topic::edit_data(&client, id)
            .await
            .map_err(log_error(handler_name))?;
        let (title, summary, markdown) = (tmp.title, tmp.summary, tmp.markdown);
        Ok(Json(
            json!({"title": title, "summary": summary, "markdown": markdown}),
        ))
    }
}
