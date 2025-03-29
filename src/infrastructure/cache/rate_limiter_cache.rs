use redis::{AsyncCommands, Client};

use crate::infrastructure::cache::error::CacheError;

pub struct RateLimitCache {
    client: Client,
}

impl RateLimitCache {
    pub async fn increment_request(&self, ip: &str) -> Result<u32, CacheError> {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        let key = format!("rate_limit:{}", ip);

        let current_count: u32 = connection.incr(&key, 1).await?;

        if current_count == 1 {
            connection.expire::<&str, ()>(&key, 60).await?; // 1-minute window
        }

        Ok(current_count)
    }
}
