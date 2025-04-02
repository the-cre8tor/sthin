use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::features::urls::{
    errors::UrlError,
    models::Url,
    value_objects::{ShortCode, ValidUrl},
};

#[derive(Debug, Clone, FromRow)]
pub struct UrlEntity {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UrlEntity {
    pub fn to_domain(&self) -> Result<Url, UrlError> {
        Ok(Url {
            id: Some(self.id),
            original_url: ValidUrl::new(self.original_url.clone())?,
            short_code: ShortCode::new(Some(self.short_code.clone()))?,
            created_at: Some(self.created_at),
            updated_at: Some(self.updated_at),
        })
    }

    pub fn from_domain(domain_url: &Url) -> Self {
        Self {
            id: domain_url.id.unwrap_or_else(|| Uuid::new_v4()),
            original_url: String::from(domain_url.original_url.as_ref()),
            short_code: String::from(domain_url.short_code.as_ref()),
            created_at: domain_url.created_at.unwrap_or_else(Utc::now),
            updated_at: domain_url.updated_at.unwrap_or_else(Utc::now),
        }
    }
}
