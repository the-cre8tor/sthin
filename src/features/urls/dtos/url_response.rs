use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{configuration::Configs, features::urls::models::Url};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlResponseDto {
    pub id: String,
    pub original_url: String,
    pub short_code: String,
    pub short_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Url> for UrlResponseDto {
    fn from(value: Url) -> Self {
        let config = Configs::get().expect("Failed to read configuration");
        let short_url = format!("{}/{}", config.application.host, value.short_code.as_str());

        Self {
            id: value.id.unwrap_or_else(Uuid::new_v4).to_string(),
            original_url: value.original_url.into_inner(),
            short_code: value.short_code.into_inner(),
            short_url,
            created_at: value.created_at.unwrap_or_else(Utc::now),
            updated_at: value.created_at.unwrap_or_else(Utc::now),
        }
    }
}
