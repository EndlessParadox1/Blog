use crate::{error::AppError, Result};
use redis::{aio::Connection, AsyncCommands};

const SESSION_KEY_PREFIX: &str = "BLOG_SESSION:";

pub async fn set_session(conn: &mut Connection, session_id: &str, value: &str) -> Result<()> {
    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
    conn.set_ex(redis_key, value, 86400) // one day
        .await
        .map_err(AppError::from)?;
    Ok(())
}

pub async fn del_session(conn: &mut Connection, session_id: &str) -> Result<()> {
    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
    conn.del(redis_key).await.map_err(AppError::from)?;
    Ok(())
}

pub async fn get_session(conn: &mut Connection, session_id: &str) -> Result<Option<String>> {
    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
    let tmp: Option<String> = conn.get(redis_key).await.map_err(AppError::from)?;
    Ok(tmp)
}
