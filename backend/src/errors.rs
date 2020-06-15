use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};

use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    BlockingError(String),
    NotFound(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::CannotEncodeJwtToken(s) => write!(f, "CannotEncodeJwtToken({})", s),
            ApiError::CannotDecodeJwtToken(s) => write!(f, "CannotDecodeJwtToken({})", s),
            ApiError::NotFound(s) => write!(f, "NotFound({})", s),
            ApiError::BlockingError(s) => write!(f, "BlockingError({})", s),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::BlockingError("Thread blocking error".to_string()),
        }
    }
}
