use crate::{
    error::AppError,
    rds::{get_session, is_user},
    session::get_session_id,
    Result,
};
use axum::http::HeaderMap;
use redis::aio::Connection;

pub async fn protect(
    headers: &HeaderMap,
    conn: &mut Connection,
    username: &str,
) -> Result<(String, String)> {
    let res = is_user(conn, username).await?;
    if !res {
        return Err(AppError::bad_request("User non-existent!"));
    }
    if let Some(session_id) = get_session_id(headers) {
        let tmp = get_session(conn, &session_id).await?;
        if let Some(session) = tmp {
            if session == username {
                return Ok((session_id, session));
            }
        }
    };
    Err(AppError::unauthorized("Please sign in!"))
}
