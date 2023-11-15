// Module declaration
pub mod config;
pub mod db;
pub mod error;
pub mod form;
pub mod handler;
mod md;
pub mod model;
mod password;
pub mod rds;
mod session;

/// Shared state
#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
    pub rdc: redis::Client,
}

pub type Result<T> = std::result::Result<T, error::AppError>;
