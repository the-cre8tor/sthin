use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct UrlStatsEntity {
    pub id: Uuid,
    pub url_id: Uuid,
    pub access_count: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
