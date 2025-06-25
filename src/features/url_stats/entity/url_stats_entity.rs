use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::features::url_stats::model::UrlStatsModel;

#[derive(Debug, Clone, FromRow)]
pub struct UrlStatsEntity {
    pub id: Uuid,
    pub url_id: Uuid,
    pub access_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UrlStatsEntity {
    pub fn from_domain(stats: UrlStatsModel) -> Self {
        Self {
            id: stats.id.unwrap_or_else(Uuid::new_v4),
            url_id: stats.url_id,
            access_count: stats.access_count,
            created_at: stats.created_at.unwrap_or_else(Utc::now),
            updated_at: stats.updated_at.unwrap_or_else(Utc::now),
            deleted_at: stats.deleted_at,
        }
    }

    pub fn to_domain(&self) -> UrlStatsModel {
        UrlStatsModel {
            id: Some(self.id),
            url_id: self.url_id,
            access_count: self.access_count,
            created_at: Some(self.created_at),
            updated_at: Some(self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}
