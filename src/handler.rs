use crate::{error::AppError, session::set_session_id, AppState, Result};
use axum::http::{header, HeaderMap, StatusCode};
use deadpool_postgres::Client;
use redis::aio::Connection;

mod auth;
pub mod backend;
pub mod frontend;
pub mod login;
pub mod register;
pub mod topic;

type RedirectView = (StatusCode, HeaderMap);

async fn get_client(state: &AppState) -> Result<Client> {
    state.pool.get().await.map_err(AppError::from)
}

async fn get_conn(state: &AppState) -> Result<Connection> {
    state
        .rdc
        .get_async_connection()
        .await
        .map_err(AppError::from)
}

fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("操作失败: {:?}, {}", err, handler_name);
        err
    })
}

fn redirect_with_session(url: &str, c: Option<&str>) -> Result<RedirectView> {
    let mut hm = match c {
        Some(s) => set_session_id(s),
        None => HeaderMap::new(),
    };
    hm.insert(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, hm))
}

fn redirect(url: &str) -> Result<RedirectView> {
    redirect_with_session(url, None)
}
