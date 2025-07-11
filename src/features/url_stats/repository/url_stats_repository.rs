use std::sync::Arc;
use uuid::Uuid;

// new api's

use crate::{
    features::{
        url_stats::{
            entity::{UrlStatsEntity, UrlStatsReportEntity},
            error::UrlStatsError,
            model::{Log, LogList, UrlStatsModel},
            queue::StatsEvent,
        },
        urls::value_objects::ShortCode,
    },
    infrastructure::database::connection::DatabasePool,
};

pub trait IUrlStatsRepository: Send + Sync {
    fn save(
        &self,
        event: &StatsEvent,
        access_count: i32,
    ) -> impl Future<Output = Result<UrlStatsModel, UrlStatsError>> + Send;

    fn find_one(&self, url_id: Uuid) -> impl Future<Output = Result<i32, UrlStatsError>> + Send;
    fn fetch_stats(
        &self,
        short_code: ShortCode,
    ) -> impl Future<Output = Result<Option<LogList>, UrlStatsError>> + Send;
}

pub struct UrlStatsRepository {
    database: Arc<DatabasePool>,
}

impl UrlStatsRepository {
    pub fn new(database: Arc<DatabasePool>) -> Self {
        Self { database }
    }
}

impl IUrlStatsRepository for UrlStatsRepository {
    async fn find_one(&self, url_id: Uuid) -> Result<i32, UrlStatsError> {
        let response = sqlx::query!(
            "SELECT access_count FROM url_stats WHERE url_id = $1",
            url_id
        )
        .fetch_optional(&self.database.pool)
        .await?;

        if let Some(record) = response {
            return Ok(record.access_count);
        }

        Ok(0)
    }

    async fn save(
        &self,
        event: &StatsEvent,
        access_count: i32,
    ) -> Result<UrlStatsModel, UrlStatsError> {
        let url_id = event.url.id.ok_or(UrlStatsError::MissingUrlId)?;
        let mut tx = self.database.pool.begin().await?;

        let result = async {
            let response = sqlx::query_as!(
                UrlStatsEntity,
                r#"
                INSERT INTO url_stats (url_id, access_count)
                VALUES ($1, $2)
                ON CONFLICT (url_id) DO UPDATE
                SET access_count = EXCLUDED.access_count,
                    updated_at = NOW()
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
            .fetch_one(&mut *tx)
            .await?;

            sqlx::query!(
                r#"
                INSERT INTO url_stats_logs (url_stats_id, ip_address, user_agent)
                VALUES ($1, $2, $3)
                "#,
                response.id,
                event.ip_address,
                event.user_agent
            )
            .execute(&mut *tx)
            .await?;

            Ok::<UrlStatsEntity, UrlStatsError>(response)
        }
        .await;

        match result {
            Ok(response) => {
                tx.commit().await?;
                Ok(response.to_domain())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    async fn fetch_stats(&self, short_code: ShortCode) -> Result<Option<LogList>, UrlStatsError> {
        let response = sqlx::query_as!(
            UrlStatsReportEntity,
            r#"
            SELECT
              url.id AS id,
              url.original_url,
              url.short_code,
              stats.access_count,
              logs.ip_address,
              logs.user_agent,
              logs.accessed_at
            FROM urls url
            JOIN url_stats stats ON stats.url_id = url.id
            LEFT JOIN url_stats_logs logs ON logs.url_stats_id = stats.id
            WHERE url.short_code = $1;
            "#,
            short_code.as_str(),
        )
        .fetch_all(&self.database.pool)
        .await?;

        if let Some(stat) = response.first() {
            let mut capt = LogList {
                id: stat.id,
                original_url: stat.original_url.clone(),
                short_code: stat.short_code.clone(),
                access_count: stat.access_count,
                logs: [].to_vec(),
            };

            for stat in response {
                let value = Log {
                    ip_address: stat.ip_address,
                    user_agent: stat.user_agent,
                    access_at: stat.accessed_at,
                };

                capt.logs.push(value);
            }

            return Ok(Some(capt));
        }

        Ok(None)
    }
}
