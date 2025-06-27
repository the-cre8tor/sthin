use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrlStatsError {
    #[error("Missing URL ID in event")]
    MissingUrlId,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal system error")]
    Unexpected(#[from] anyhow::Error),
}
