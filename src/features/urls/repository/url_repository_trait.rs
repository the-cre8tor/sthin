use uuid::Uuid;

use crate::features::urls::errors::DomainError;
use crate::features::urls::models::Url;
use crate::features::urls::value_objects::{ShortCode, ValidUrl};

pub trait IUrlRepository: Send + Sync {
    fn save(&self, url: &Url) -> impl Future<Output = Result<Url, DomainError>> + Send;
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Uuid>, DomainError>> + Send;
    fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> impl Future<Output = Result<Option<Url>, DomainError>> + Send; // update the short code type
    fn find_by_original_url(
        &self,
        original_url: &ValidUrl,
    ) -> impl Future<Output = Result<Option<Url>, DomainError>> + Send;
    // fn update(&self, url: &Url) -> impl Future<Output = Result<Url, DomainError>> + Send;
    fn delete_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;
    fn exists_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;
}
