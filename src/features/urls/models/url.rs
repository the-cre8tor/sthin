use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::features::urls::value_objects::{ShortCode, ValidUrl};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Url {
    pub id: Option<Uuid>,
    pub original_url: ValidUrl,
    pub short_code: ShortCode,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Url {
    pub fn new(original_url: ValidUrl, short_code: ShortCode) -> Self {
        Self {
            id: None,
            original_url,
            short_code,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }

    pub fn update_url(&mut self, new_url: ValidUrl) {
        self.original_url = new_url;
        self.updated_at = Some(Utc::now())
    }
}
