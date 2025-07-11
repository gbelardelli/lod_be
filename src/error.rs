use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    DbError(sqlx::Error),
    BadRequest(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::DbError(err) => write!(f, "Database error: {}", err),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_body = match self {
            AppError::NotFound(msg) => ErrorResponse {
                error: "not_found".to_string(),
                message: msg.clone(),
            },
            AppError::BadRequest(msg) => ErrorResponse {
                error: "bad_request".to_string(),
                message: msg.clone(),
            },
            AppError::DbError(err) => ErrorResponse {
                error: "db_error".to_string(),
                message: err.to_string(),
            },
            AppError::Internal(msg) => ErrorResponse {
                error: "internal".to_string(),
                message: msg.clone(),
            },
        };

        let status_code = match self {
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::DbError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(error_body)
    }
}
