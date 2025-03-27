// use chrono::{DateTime, Utc};
// use sqlx::FromRow;
// use uuid::Uuid;

// #[derive(Debug, Clone, FromRow)]
// pub struct DbUrlAccess {
//     pub id: Uuid,
//     pub url_id: Uuid,
//     pub accessed_at: DateTime<Utc>,
//     pub ip_address: Option<String>,
//     pub user_agent: Option<String>,
//     pub referrer: Option<String>,
// }

// impl DbUrlAccess {
//     pub fn to_domain(&self) -> UrlAccess {
//         UrlAccess {
//             id: Some(self.id),
//             url_id: self.url_id,
//             accessed_at: self.accessed_at,
//             ip_address: self.ip_address.clone(),
//             user_agent: self.user_agent.clone(),
//             referrer: self.referrer.clone(),
//         }
//     }

//     pub fn from_domain(domain_access: &UrlAccess) -> Self {
//         Self {
//             id: domain_access.id.unwrap_or_else(|| Uuid::new_v4()),
//             url_id: domain_access.url_id,
//             accessed_at: domain_access.accessed_at,
//             ip_address: domain_access.ip_address.clone(),
//             user_agent: domain_access.user_agent.clone(),
//             referrer: domain_access.referrer.clone(),
//         }
//     }
// }
