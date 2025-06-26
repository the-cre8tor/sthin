use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlStatsReportModel {
    pub id: Option<Uuid>,
    pub original_url: String,
    pub short_code: String,
    pub access_count: i32,
    pub ip_address: String,
    pub user_agent: String,
    pub accessed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct LogList {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub access_count: i32,
    pub logs: Vec<Log>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Log {
    pub ip_address: String,
    pub user_agent: String,
    pub access_at: DateTime<Utc>,
}
