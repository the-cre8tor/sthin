use axum::response::{IntoResponse, Response};

use hyper::StatusCode;
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(errors) => {
                ApiResponse::<Value>::fail(json!({ "validation": errors }), StatusCode::BAD_REQUEST)
            }
            AppError::NotFound(msg) => {
                ApiResponse::<Value>::fail(json!({ "message": msg }), StatusCode::NOT_FOUND)
            }
            AppError::Database(err) => {
                println!("Database error: {}", err);
                ApiResponse::<&str>::error("A database error occurred")
            }
            _ => ApiResponse::<&str>::error("An internal error occurred"),
        }
    }
}
