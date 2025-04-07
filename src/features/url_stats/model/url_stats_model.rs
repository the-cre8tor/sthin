use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlStats {
    pub id: Option<Uuid>,
    pub url_id: Uuid,
    pub access_count: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UrlStats {
    pub fn new(url_id: Uuid, access_count: i32) -> Self {
        Self {
            id: None,
            url_id,
            access_count,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }

    pub fn update_access_count(&mut self) {
        self.access_count = self.access_count + 1;
        self.updated_at = Some(Utc::now())
    }
}
