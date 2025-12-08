use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Database error")]
    DbError(#[from] sqlx::Error),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            ApiError::DbError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}