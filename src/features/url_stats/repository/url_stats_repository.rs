use sqlx::PgPool;
use uuid::Uuid;

use crate::features::url_stats::{entity::UrlStatsEntity, error::UrlStatsError};

pub trait IUrlStatsRepository {
    fn save(
        &self,
        url_id: Uuid,
        access_count: i32,
    ) -> impl Future<Output = Result<UrlStatsEntity, UrlStatsError>> + Send;
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
    async fn save(&self, url_id: Uuid, access_count: i32) -> Result<UrlStatsEntity, UrlStatsError> {
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

        Ok(response)
    }
}
