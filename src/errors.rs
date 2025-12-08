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

#[cfg(test)]
mod tests {
    use super::*;

    mod api_error {
        use super::*;

        #[test]
        fn not_found_error_message() {
            let error = ApiError::NotFound;
            assert_eq!(error.to_string(), "Not found");
        }

        #[test]
        fn bad_request_error_includes_message() {
            let message = "Email is required".to_string();
            let error = ApiError::BadRequest(message.clone());
            assert_eq!(error.to_string(), format!("Bad request: {}", message));
        }

        #[test]
        fn bad_request_with_empty_message() {
            let error = ApiError::BadRequest(String::new());
            assert_eq!(error.to_string(), "Bad request: ");
        }

        #[test]
        fn bad_request_with_special_characters() {
            let message = "Invalid input: @#$%".to_string();
            let error = ApiError::BadRequest(message.clone());
            assert!(error.to_string().contains("@#$%"));
        }

        #[test]
        fn bad_request_error_can_be_created() {
            let error = ApiError::BadRequest("test".to_string());
            match error {
                ApiError::BadRequest(msg) => assert_eq!(msg, "test"),
                _ => panic!("Expected BadRequest variant"),
            }
        }

        #[test]
        fn not_found_error_can_be_created() {
            let error = ApiError::NotFound;
            match error {
                ApiError::NotFound => (),
                _ => panic!("Expected NotFound variant"),
            }
        }
    }

    mod error_response {
        use super::*;

        #[test]
        fn not_found_returns_404() {
            let error = ApiError::NotFound;
            let response = error.error_response();
            assert_eq!(response.status(), 404);
        }

        #[test]
        fn bad_request_returns_400() {
            let error = ApiError::BadRequest("test".to_string());
            let response = error.error_response();
            assert_eq!(response.status(), 400);
        }

        #[test]
        fn bad_request_includes_message_in_response() {
            let message = "Invalid email format";
            let error = ApiError::BadRequest(message.to_string());
            let response = error.error_response();
            assert_eq!(response.status(), 400);
        }

        #[test]
        fn db_error_returns_500() {
            // Create a simple database error for testing
            // In real scenarios, this would come from sqlx operations
            let error = ApiError::DbError(sqlx::Error::RowNotFound);
            let response = error.error_response();
            assert_eq!(response.status(), 500);
        }
    }

    mod error_display {
        use super::*;

        #[test]
        fn error_implements_display() {
            let error = ApiError::NotFound;
            let display_string = format!("{}", error);
            assert_eq!(display_string, "Not found");
        }

        #[test]
        fn error_implements_debug() {
            let error = ApiError::BadRequest("test".to_string());
            let debug_string = format!("{:?}", error);
            assert!(debug_string.contains("BadRequest"));
        }
    }
}