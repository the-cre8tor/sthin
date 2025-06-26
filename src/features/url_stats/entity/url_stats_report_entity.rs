use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::features::url_stats::model::UrlStatsReportModel;

#[derive(Debug, Clone, FromRow)]
pub struct UrlStatsReportEntity {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub access_count: i32,
    pub ip_address: String,
    pub user_agent: String,
    pub accessed_at: DateTime<Utc>,
}

impl UrlStatsReportEntity {
    pub fn from_domain(model: UrlStatsReportModel) -> Self {
        Self {
            id: model.id.unwrap_or_else(Uuid::new_v4),
            original_url: model.original_url,
            short_code: model.short_code,
            access_count: model.access_count,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            accessed_at: model.accessed_at.unwrap_or_else(Utc::now),
        }
    }

    pub fn to_domain(self) -> UrlStatsReportModel {
        UrlStatsReportModel {
            id: Some(self.id),
            original_url: self.original_url,
            short_code: self.short_code,
            access_count: self.access_count,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            accessed_at: Some(self.accessed_at),
        }
    }
}
