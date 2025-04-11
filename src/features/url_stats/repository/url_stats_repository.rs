use sqlx::PgPool;
use uuid::Uuid;

use crate::features::url_stats::{entity::UrlStatsEntity, error::UrlStatsError, model::UrlStats};

pub trait IUrlStatsRepository: Send + Sync {
    fn save(
        &self,
        url_id: Uuid,
        access_count: i32,
    ) -> impl Future<Output = Result<UrlStats, UrlStatsError>> + Send;

    fn find_one(&self, url_id: Uuid) -> impl Future<Output = Result<i32, UrlStatsError>> + Send;
}

pub struct UrlStatsRepository {
    client: PgPool,
}

impl UrlStatsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { client: pool }
    }
}

impl IUrlStatsRepository for UrlStatsRepository {
    async fn save(&self, url_id: Uuid, access_count: i32) -> Result<UrlStats, UrlStatsError> {
        let response = sqlx::query_as!(
            UrlStatsEntity,
            r#"
            INSERT INTO url_stats (url_id, access_count)
            VALUES ($1, $2)
            ON CONFLICT (url_id) DO UPDATE
            SET access_count = EXCLUDED.access_count,
                updated_at = EXCLUDED.updated_at
            RETURNING id as "id!",
                      url_id as "url_id!",
                      access_count as "access_count!",
                      created_at as "created_at!",
                      updated_at as "updated_at!",
                      deleted_at
            "#,
            url_id,
            access_count
        )
        .fetch_one(&self.client)
        .await?;

        response.to_domain()
    }

    async fn find_one(&self, url_id: Uuid) -> Result<i32, UrlStatsError> {
        let response = sqlx::query!(
            "SELECT access_count FROM url_stats WHERE url_id = $1",
            url_id
        )
        .fetch_optional(&self.client)
        .await?;

        if let Some(record) = response {
            return Ok(record.access_count);
        }

        Ok(0)
    }
}
