use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::features::url_stats::model::UrlStatsLogsModel;

#[derive(Debug, Clone, FromRow)]
pub struct UrlStatsLog {
    pub id: Uuid,
    pub url_stats_id: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub accessed_at: DateTime<Utc>,
}

impl UrlStatsLog {
    pub fn from_domain(model: UrlStatsLogsModel) -> Self {
        Self {
            id: model.id.unwrap_or_else(Uuid::new_v4),
            url_stats_id: model.url_stats_id,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            accessed_at: model.accessed_at.unwrap_or_else(Utc::now),
        }
    }

    pub fn to_domain(self) -> UrlStatsLogsModel {
        UrlStatsLogsModel {
            id: Some(self.id),
            url_stats_id: self.url_stats_id,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            accessed_at: Some(self.accessed_at),
        }
    }
}
