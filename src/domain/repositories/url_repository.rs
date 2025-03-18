use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{DomainError, ShortCode, Url, ValidUrl};

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn save(&self, url: &Url) -> Result<Url, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Uuid>, DomainError>;
    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Option<Url>, DomainError>; // update the short code type
    async fn find_by_original_url(
        &self,
        original_url: &ValidUrl,
    ) -> Result<Option<Url>, DomainError>;
    async fn update(&self, url: &Url) -> Result<Url, DomainError>;
    async fn delete_by_short_code(&self, short_code: &ShortCode) -> Result<bool, DomainError>;
    async fn exists_by_short_code(&self, short_code: &ShortCode) -> Result<bool, DomainError>;
}
