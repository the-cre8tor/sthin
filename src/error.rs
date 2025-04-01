use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use sqlx::Error as SQLxError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    // Core error types (similar to previous implementation)
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found")]
    NotFound,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Forbidden action")]
    Forbidden,

    // Database errors
    #[error("Database connection error: {0}")]
    DatabaseConnectionError(String),

    #[error("Database query error: {0}")]
    DatabaseQueryError(String),

    #[error("Unique constraint violation: {0}")]
    UniqueConstraintViolation(String),

    // Infrastructure errors
    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // Serialization errors
    #[error("Serialization error: {0}")]
    SerializationError(String),

    // Internal server errors
    #[error("Internal server error: {0}")]
    InternalError(String),

    // Domain-specific errors
    #[error("URL generation failed")]
    UrlGenerationFailed,

    #[error("Invalid URL format")]
    InvalidUrlFormat,

    #[error("Short code generation failed")]
    ShortCodeGenerationFailed,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Validation Error",
                "message": msg
            })),

            AppError::NotFound => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": AppError::NotFound.to_string()
            })),

            AppError::DatabaseQueryError(err) => {
                // log::error!("Database error: {:?}", err);
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": "A database error occurred"
                }))
            }
            AppError::UnexpectedError(msg) => {
                // log::error!("Internal error: {}", msg);
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": "An internal error occurred"
                }))
            }
            _ => HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "message": "An internal error occurred"
            })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::DatabaseQueryError(_) | AppError::UnexpectedError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Error conversion and utility methods
// impl AppError {
//     /// Convert error to an HTTP status code
//     pub fn status_code(&self) -> StatusCode {
//         match self {
//             Self::ValidationError(_) => StatusCode::BAD_REQUEST,
//             Self::NotFound => StatusCode::NOT_FOUND,
//             Self::Unauthorized => StatusCode::UNAUTHORIZED,
//             Self::Forbidden => StatusCode::FORBIDDEN,
//             Self::UniqueConstraintViolation(_) => StatusCode::CONFLICT,
//             Self::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }

//     /// Generate a structured error response
//     pub fn error_response(&self) -> serde_json::Value {
//         match self {
//             Self::ValidationError(msg) => json!({
//                 "type": "validation_error",
//                 "message": msg
//             }),
//             Self::NotFound => json!({
//                 "type": "not_found",
//                 "message": "The requested resource could not be found"
//             }),
//             _ => json!({
//                 "type": "error",
//                 "message": self.to_string()
//             }),
//         }
//     }
// }

// Conversion traits for various error types
impl From<SQLxError> for AppError {
    fn from(error: SQLxError) -> Self {
        match error {
            SQLxError::RowNotFound => AppError::NotFound,
            SQLxError::Database(db_error) => {
                // Specific database error handling
                if let Some(code) = db_error.code() {
                    match code.as_ref() {
                        "23505" => AppError::UniqueConstraintViolation(db_error.to_string()),
                        _ => AppError::DatabaseQueryError(db_error.to_string()),
                    }
                } else {
                    AppError::DatabaseQueryError(db_error.to_string())
                }
            }
            SQLxError::PoolTimedOut | SQLxError::PoolClosed => {
                AppError::DatabaseConnectionError(error.to_string())
            }
            _ => AppError::InternalError(error.to_string()),
        }
    }
}

// Conversion for validation errors
impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, field_errors)| {
                field_errors
                    .iter()
                    .map(|error| format!("{}: {}", field, error))
                    .collect::<Vec<String>>()
            })
            .collect();

        AppError::ValidationError(error_messages.join(", "))
    }
}

// Logging trait for errors
pub trait ErrorLogging {
    fn log(&self);
}

impl ErrorLogging for AppError {
    fn log(&self) {
        match self {
            AppError::InternalError(msg) | AppError::DatabaseQueryError(msg) => {
                tracing::error!("Critical error: {}", msg)
            }
            AppError::ValidationError(msg) => {
                tracing::warn!("Validation error: {}", msg)
            }
            _ => tracing::debug!("Error occured: {}", self),
        }
    }
}

// Utility for result type with AppError
pub type Result<T> = std::result::Result<T, AppError>;

pub trait ErrorContext<T> {
    fn context(self, message: &str) -> Result<T>;
}

impl<T> ErrorContext<T> for std::result::Result<T, AppError> {
    fn context(self, message: &str) -> Result<T> {
        self.map_err(|mut err| {
            match &mut err {
                AppError::InternalError(msg) => {
                    msg.push_str(&format!(". Context: {}", message));
                }
                _ => {}
            }

            err
        })
    }
}
