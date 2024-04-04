use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum AppErrorType {
    Redis,
    Db,
    Crypt,
    BadRequest,
    NotFound,
    Unauthorized,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn Error>>,
    pub types: AppErrorType,
}

impl AppError {
    fn new(message: Option<String>, cause: Option<Box<dyn Error>>, types: AppErrorType) -> Self {
        Self {
            message,
            cause,
            types,
        }
    }

    fn from_err(cause: Box<dyn Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }

    fn from_str(msg: &str, types: AppErrorType) -> Self {
        Self::new(Some(msg.to_string()), None, types)
    }

    pub fn bad_request(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::BadRequest)
    }

    pub fn unauthorized(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::Unauthorized)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AppError {}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Crypt)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Redis)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = match self.types {
            AppErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            AppErrorType::BadRequest => StatusCode::BAD_REQUEST,
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let msg = self
            .message
            .to_owned()
            .unwrap_or("Something went wrong!".to_string());
        (code, Json(json!({"msg": msg}))).into_response()
    }
}
