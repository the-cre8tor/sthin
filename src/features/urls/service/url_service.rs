use std::sync::Arc;

use crate::features::urls::errors::UrlError;
use crate::features::urls::models::Url;
use crate::features::urls::repository::IUrlRepository;
use crate::features::urls::value_objects::{ShortCode, ValidUrl};

pub trait IUrlService: Send + Sync {
    fn create_short_url(
        &self,
        original_url: ValidUrl,
        short_code: Option<ShortCode>,
    ) -> impl Future<Output = Result<Url, UrlError>> + Send;

    fn get_url_by_short_code(
        &self,
        short_code: ShortCode,
    ) -> impl Future<Output = Result<Url, UrlError>> + Send;

    fn update_url_by_short_code(
        &self,
        short_code: ShortCode,
        valid_url: ValidUrl,
    ) -> impl Future<Output = Result<Url, UrlError>> + Send;

    fn delete_url_by_short_code(
        &self,
        url: &ShortCode,
    ) -> impl Future<Output = Result<bool, UrlError>> + Send;
}

#[derive(Clone)]
pub struct UrlService<R: IUrlRepository> {
    url_repo: Arc<R>,
}

impl<R: IUrlRepository> UrlService<R> {
    pub fn new(url_repository: Arc<R>) -> Self {
        Self {
            url_repo: url_repository,
        }
    }
}

impl<R: IUrlRepository> IUrlService for UrlService<R> {
    async fn create_short_url(
        &self,
        original_url: ValidUrl,
        short_code: Option<ShortCode>,
    ) -> Result<Url, UrlError> {
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
                return Err(UrlError::Duplicate(String::from(
                    "We currently can't find a unique short code for you, please try again",
                )));
            }

            short_code = ShortCode::new(None)?;
            attempts += 1;
        }

        let url = Url::new(original_url, short_code);
        let created_url = self.url_repo.save(&url).await?;

        Ok(created_url)
    }

    async fn get_url_by_short_code(&self, short_code: ShortCode) -> Result<Url, UrlError> {
        let result = self.url_repo.find_by_short_code(&short_code).await?;

        if let Some(url) = result {
            Ok(url)
        } else {
            let msg = format!(
                "We're unable to find any url link to this short code: {}",
                short_code.as_str()
            );
            Err(UrlError::NotFound(msg))
        }
    }

    async fn update_url_by_short_code(
        &self,
        short_code: ShortCode,
        valid_url: ValidUrl,
    ) -> Result<Url, UrlError> {
        let mut url = self.get_url_by_short_code(short_code).await?;

        self.url_repo.update(&mut url, valid_url).await
    }

    async fn delete_url_by_short_code(&self, short_code: &ShortCode) -> Result<bool, UrlError> {
        let url = self.url_repo.find_by_short_code(short_code).await?;

        if url.is_none() {
            return Err(UrlError::NotFound(format!(
                "URL with the associated shortcode: '{}' is not found!",
                short_code.as_str()
            )));
        }

        self.url_repo.delete_by_short_code(short_code).await
    }
}
