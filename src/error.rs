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

#[derive(Debug, PartialEq)]
pub enum AppErrorType {
    NotFound,
    Db,
    Chrono,
    Crypt,
    IncorrectLogin,
    IncorrectRegister,
    Unauthorized,
    Redis,
    Duplication,
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

    pub fn notfound_opt(message: Option<String>) -> Self {
        Self::new(message, None, AppErrorType::NotFound)
    }

    pub fn incorrect_login() -> Self {
        Self::from_str(
            "Username or password incorrect!",
            AppErrorType::IncorrectLogin,
        )
    }

    pub fn multi_register() -> Self {
        Self::from_str("Username existed!", AppErrorType::IncorrectRegister)
    }

    pub fn bad_register() -> Self {
        Self::from_str(
            "Content submitted incorrect!",
            AppErrorType::IncorrectRegister,
        )
    }

    pub fn unauthorized() -> Self {
        Self::from_str("Unauthorized, please sign in!", AppErrorType::Unauthorized)
    }

    pub fn duplication() -> Self {
        Self::from_str("Repetitive action!", AppErrorType::Duplication)
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
impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Chrono)
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
            AppErrorType::IncorrectLogin
            | AppErrorType::IncorrectRegister
            | AppErrorType::Duplication => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let msg = self
            .message
            .to_owned()
            .unwrap_or("Something wrong happened!".to_string());
        (code, Json(json!({"msg": msg}))).into_response()
    }
}
