use super::error::CacheError;
use redis::{AsyncCommands, Client, ConnectionLike};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

#[derive(Debug)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    #[tracing::instrument(
        name = "Redis client connecting",
        skip(redis_url),
        fields(is_redis_connected=tracing::field::Empty)
    )]
    pub fn new(redis_url: &str) -> Result<Self, CacheError> {
        let mut client = Client::open(redis_url)?;

        let is_redis_connected = client.check_connection();

        if !is_redis_connected {
            tracing::error!("{}", CacheError::RedisConnectionError);
            return Err(CacheError::RedisConnectionError);
        }

        tracing::Span::current().record(
            "is_redis_connected",
            tracing::field::display(is_redis_connected),
        );

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

        let deserialized = result
            .map(|value| serde_json::from_str(&value).map_err(CacheError::JsonSerializationError));

        deserialized.transpose()
    }
}
