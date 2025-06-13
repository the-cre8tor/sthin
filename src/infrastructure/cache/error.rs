#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Failed to connect to Redis server")]
    RedisConnectionError,

    #[error("Failed to serialize json value: {0}")]
    JsonSerializationError(#[from] serde_json::Error),

    #[error(transparent)]
    ContextualError(#[from] anyhow::Error),
}
