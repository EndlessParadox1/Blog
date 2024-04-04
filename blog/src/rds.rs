use crate::{error::AppError, Result};
use redis::{aio::Connection, AsyncCommands};

const SESSION_KEY_PREFIX: &str = "BLOG_SESSION:";
const USER_SET: &str = "BLOG_USERS";

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
    let s: Option<String> = conn.get(redis_key).await.map_err(AppError::from)?;
    Ok(s)
}

pub async fn add_user(conn: &mut Connection, value: &str) -> Result<()> {
    conn.sadd(USER_SET, value).await.map_err(AppError::from)?;
    Ok(())
}

pub async fn is_user(conn: &mut Connection, value: &str) -> Result<bool> {
    let n: i32 = conn
        .sismember(USER_SET, value)
        .await
        .map_err(AppError::from)?;
    Ok(n > 0)
}
