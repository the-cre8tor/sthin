// use async_trait::async_trait;
// use sqlx::{PgPool, Error as SqlxError};
// use uuid::Uuid;
// use chrono::{DateTime, Utc};
// use std::collections::HashMap;

// use crate::domain::repositories::UrlAccessRepository;
// use crate::domain::errors::DomainError;
// use crate::infrastructure::database::models::DbUrlAccess;

// pub struct PgUrlAccessRepository {
//     pool: PgPool,
// }

// impl PgUrlAccessRepository {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }
// }

// #[async_trait]
// impl UrlAccessRepository for PgUrlAccessRepository {
//     async fn record_access(&self, url_id: Uuid, client_info: ClientInfo) -> Result<(), DomainError> {
//         sqlx::query!(
//             r#"
//             INSERT INTO url_accesses (url_id, accessed_at, ip_address, user_agent, referrer)
//             VALUES ($1, $2, $3, $4, $5)
//             "#,
//             url_id,
//             Utc::now(),
//             client_info.ip_address,
//             client_info.user_agent,
//             client_info.referrer
//         )
//         .execute(&self.pool)
//         .await
//         .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

//         Ok(())
//     }

//     async fn get_stats_for_url(&self, url_id: Uuid) -> Result<UrlAccessStats, DomainError> {
//         let total_accesses = sqlx::query_scalar!(
//             "SELECT COUNT(*) FROM url_accesses WHERE url_id = $1",
//             url_id
//         )
//         .fetch_one(&self.pool)
//         .await
//         .map_err(|e| DomainError::DatabaseError(e.to_string()))?
//         .unwrap_or(0);

//         let unique_visitors = sqlx::query_scalar!(
//             "SELECT COUNT(DISTINCT ip_address) FROM url_accesses WHERE url_id = $1",
//             url_id
//         )
//         .fetch_one(&self.pool)
//         .await
//         .map_err(|e| DomainError::DatabaseError(e.to_string()))?
//         .unwrap_or(0);

//         let last_accessed = sqlx::query_scalar!(
//             "SELECT MAX(accessed_at) FROM url_accesses WHERE url_id = $1",
//             url_id
//         )
//         .fetch_one(&self.pool)
//         .await
//         .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

//         let referrer_stats = sqlx::query_as!(
//             (String, i64),
//             "SELECT referrer, COUNT(*) as count
//              FROM url_accesses
//              WHERE url_id = $1 AND referrer IS NOT NULL
//              GROUP BY referrer",
//             url_id
//         )
//         .fetch_all(&self.pool)
//         .await
//         .map_err(|e| DomainError::DatabaseError(e.to_string()))?
//         .into_iter()
//         .collect::<HashMap<String, i64>>();

//         Ok(UrlAccessStats {
//             total_accesses,
//             unique_visitors,
//             last_accessed,
//             referrer_stats,
//         })
//     }
// }
