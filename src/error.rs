use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::{Value, json};
use thiserror::Error;
use validator::ValidationErrors;

use crate::infrastructure::server::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation failed")]
    Validation(String),

    #[error("DTO validation error")]
    RawValidator(#[from] ValidationErrors),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unathorized(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Validation(errors) => {
                ApiResponse::<Value>::fail(json!({"validation": errors}), StatusCode::BAD_REQUEST)
            }
            AppError::NotFound(msg) => {
                ApiResponse::<Value>::fail(json!({"message": msg}), StatusCode::NOT_FOUND)
            }
            AppError::Database(err) => {
                println!("Database error: {}", err);
                ApiResponse::<&str>::error("A database error occurred")
            }
            AppError::Internal(_msg) => ApiResponse::<&str>::error("An internal error occurred"),
            _ => ApiResponse::<&str>::error("An internal error occurred"),
        }
    }
}

// Conversion traits for various error types
// impl From<sqlx::Error> for AppError {
//     fn from(error: sqlx::Error) -> Self {
//         match error {
//             sqlx::Error::RowNotFound => AppError::NotFound,
//             sqlx::Error::Database(db_error) => {
//                 // Specific database error handling
//                 if let Some(code) = db_error.code() {
//                     match code.as_ref() {
//                         "23505" => AppError::UniqueConstraintViolation(db_error.to_string()),
//                         _ => AppError::DatabaseQueryError(db_error.to_string()),
//                     }
//                 } else {
//                     AppError::DatabaseQueryError(db_error.to_string())
//                 }
//             }
//             sqlx::Error::PoolTimedOut | SQLxError::PoolClosed => {
//                 AppError::DatabaseConnectionError(error.to_string())
//             }
//             _ => AppError::Internal(error.to_string()),
//         }
//     }
// }

// Conversion for validation errors
// impl From<ValidationErrors> for AppError {
//     fn from(errors: ValidationErrors) -> Self {
//         let error_messages: Vec<String> = errors
//             .field_errors()
//             .iter()
//             .flat_map(|(field, field_errors)| {
//                 field_errors
//                     .iter()
//                     .map(|error| format!("{}: {}", field, error))
//                     .collect::<Vec<String>>()
//             })
//             .collect();

//         AppError::Validation(error_messages.join(", "))
//     }
// }
