use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlStatsLogsModel {
    pub id: Option<Uuid>,
    pub url_stats_id: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub accessed_at: Option<DateTime<Utc>>,
}

impl UrlStatsLogsModel {
    pub fn new(url_stats_id: Uuid, ip_address: String, user_agent: String) -> Self {
        Self {
            id: None,
            url_stats_id,
            ip_address,
            user_agent,
            accessed_at: Some(Utc::now()),
        }
    }
}
