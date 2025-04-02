use uuid::Uuid;

use crate::features::urls::errors::UrlError;
use crate::features::urls::models::Url;
use crate::features::urls::value_objects::{ShortCode, ValidUrl};

pub trait IUrlRepository: Send + Sync {
    fn save(&self, url: &Url) -> impl Future<Output = Result<Url, UrlError>> + Send;
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Uuid>, UrlError>> + Send;
    fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> impl Future<Output = Result<Option<Url>, UrlError>> + Send; // update the short code type
    fn find_by_original_url(
        &self,
        original_url: &ValidUrl,
    ) -> impl Future<Output = Result<Option<Url>, UrlError>> + Send;
    // fn update(&self, url: &Url) -> impl Future<Output = Result<Url, DomainError>> + Send;
    fn delete_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> impl Future<Output = Result<bool, UrlError>> + Send;
    fn exists_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> impl Future<Output = Result<bool, UrlError>> + Send;
}
