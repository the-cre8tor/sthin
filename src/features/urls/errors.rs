use thiserror::Error;

use crate::error::AppError;

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    #[error("Invalid short code: {0}")]
    InvalidShortCode(String),

    #[error("URL too long (max {0} characters)")]
    UrlTooLong(usize),

    #[error("URL not found: {0}")]
    NotFound(String),

    #[error("URL already exists: {0}")]
    Duplicate(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<UrlError> for AppError {
    fn from(error: UrlError) -> Self {
        match error {
            UrlError::InvalidUrl(msg)
            | UrlError::InvalidShortCode(msg)
            | UrlError::Duplicate(msg) => AppError::Validation(msg),
            UrlError::NotFound(msg) => AppError::NotFound(msg),
            UrlError::UrlTooLong(len) => AppError::Validation(len.to_string()),
            UrlError::Database(error) => AppError::Database(error),
        }
    }
}
