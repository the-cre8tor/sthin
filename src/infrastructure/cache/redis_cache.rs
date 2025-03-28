use std::time::Duration;

use redis::{AsyncCommands, Client};
use serde::{Serialize, de::DeserializeOwned};

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, CacheError> {
        let client = Client::open(redis_url)?;
        Ok(RedisCache { client })
    }

    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        expiration: Option<Duration>,
    ) -> Result<(), CacheError> {
        let mut connect = self.client.get_multiplexed_async_connection().await?;
        let serialized = serde_json::to_string(value)?;

        match expiration {
            Some(duration) => connect.set_ex(key, serialized, duration.as_secs()).await?,
            None => connect.set(key, serialized).await?,
        }

        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, CacheError> {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = connection.get(key).await?;

        let deserialized = result.map(|value| {
            let value = value.to_string();
            serde_json::from_str(&value).map_err(CacheError::JsonSerializationError)
        });

        deserialized.transpose()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Failed to serialize json value: {0}")]
    JsonSerializationError(#[from] serde_json::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("This is a simple error handling")]
    Simple,
}
