use crate::{error::AppError, AppState, Result};
use deadpool_postgres::Client;
use redis::aio::Connection;

pub mod admin;
mod auth;
pub mod front;
pub mod login;
pub mod register;
pub mod topic;

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
