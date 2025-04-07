use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrlStatsError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Internal system error")]
    Unexpected(#[from] anyhow::Error),
}
