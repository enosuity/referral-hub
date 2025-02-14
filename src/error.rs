use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(sqlx::Error),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(json_error("Database error"))
            }
            AppError::CacheError(_) => {
                HttpResponse::InternalServerError().json(json_error("Cache error"))
            }
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json_error(msg))
            }
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(json_error(msg))
            }
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(json_error("Internal server error"))
            }
        }
    }
}

fn json_error(message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": message
    })
} 