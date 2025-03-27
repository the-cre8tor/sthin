use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid URL format")]
    InvalidUrl,

    #[error("URL too long (max {0} characters)")]
    UrlTooLong(usize),

    #[error("Shortcode already exists")]
    ShortcodeConflict,

    #[error("Domain validation error: {0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] sqlx::Error),
}
