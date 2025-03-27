use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct DbUrlAccess {
    pub id: Uuid,
    pub url_id: Uuid,
    pub accessed_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
}

impl DbUrlAccess {
    pub fn to_domain(&self) -> () {}

    pub fn from_domain(_domain_access: Option<String>) {}
}
