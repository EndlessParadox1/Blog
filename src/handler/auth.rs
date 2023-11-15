use crate::{error::AppError, rds::get_session, session::get_session_id, Result};
use axum::http::HeaderMap;
use redis::aio::Connection;

pub async fn protect(headers: &HeaderMap, conn: &mut Connection, username: &str) -> Result<()> {
    if let Some(session_id) = get_session_id(headers) {
        let tmp = get_session(conn, &session_id).await?;
        if let Some(session) = tmp {
            if session == username {
                return Ok(());
            }
        }
    };
    Err(AppError::unauthorized())
}
