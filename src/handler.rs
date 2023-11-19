use crate::{error::AppError, AppState, Result, session::set_session_id};
use deadpool_postgres::Client;
use redis::aio::Connection;
use axum::http::{header, HeaderMap, StatusCode};

pub mod admin;
mod auth;
pub mod front;
pub mod login;
pub mod register;
pub mod topic;

type RedirectView = (StatusCode, HeaderMap, ());

// get client from pg pool
async fn get_client(state: &AppState) -> Result<Client> {
    state.pool.get().await.map_err(AppError::from)
}

// get connection from redis client
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
        tracing::error!("Handler failed: {:?}, {}", err, handler_name);
        err
    })
}

fn redirect_with_session(url: &str, c:Option<&str>) -> Result<RedirectView> {
    let mut hm = match c {
        Some(s) => set_session_id(s),
        None => HeaderMap::new(),
    };
    hm.insert(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, hm, ()))
}

fn redirect(url: &str) -> Result<RedirectView> {
    redirect_with_session(url, None)
}
