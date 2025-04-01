use crate::features::urls::errors::DomainError;
use crate::features::urls::models::Url;
use crate::features::urls::repository::IUrlRepository;
use crate::features::urls::value_objects::{ShortCode, ValidUrl};

pub trait IUrlService: Send + Sync {
    fn create_short_url(
        &self,
        original_url: ValidUrl,
        short_code: Option<ShortCode>,
    ) -> impl Future<Output = Result<Url, DomainError>> + Send;
}

#[derive(Clone)]
pub struct UrlService<R: IUrlRepository> {
    url_repo: R,
}

impl<R: IUrlRepository> IUrlService for UrlService<R> {
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

impl<R: IUrlRepository> UrlService<R> {
    pub fn new(url_repository: R) -> Self {
        Self {
            url_repo: url_repository,
        }
    }
}
