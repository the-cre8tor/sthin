use async_trait::async_trait;

use crate::domain::errors::DomainError;
use crate::domain::models::Url;
use crate::domain::repositories::UrlRepository;
use crate::domain::value_objects::{ShortCode, ValidUrl};

#[async_trait]
pub trait IUrlService: Send + Sync {
    async fn create_short_url(
        &self,
        original_url: ValidUrl,
        short_code: Option<ShortCode>,
    ) -> Result<Url, DomainError>;
}

pub struct UrlService<R: UrlRepository> {
    url_repo: R,
}

impl<R: UrlRepository> UrlService<R> {
    pub fn new(url_repository: R) -> Self {
        Self {
            url_repo: url_repository,
        }
    }
}

#[async_trait]
impl<R: UrlRepository> IUrlService for UrlService<R> {
    async fn create_short_url(
        &self,
        original_url: ValidUrl,
        short_code: Option<ShortCode>,
    ) -> Result<Url, DomainError> {
        if let Some(existing) = self.url_repo.find_by_original_url(&original_url).await? {
            return Ok(existing);
        }

        let mut short_code = match short_code {
            Some(value) => value,
            None => ShortCode::new(None)?,
        };

        let mut attempts = 0;

        while self.url_repo.exists_by_short_code(&short_code).await? {
            if attempts >= 5 {
                return Err(DomainError::ShortcodeConflict);
            }

            short_code = ShortCode::new(None)?;
            attempts += 1;
        }

        let url = Url::new(original_url, short_code);
        let created_url = self.url_repo.save(&url).await?;

        Ok(created_url)
    }
}
