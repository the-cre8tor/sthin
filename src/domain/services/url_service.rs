use crate::domain::{DomainError, ShortCode, Url, UrlRepository, ValidUrl};

pub struct UrlService<R: UrlRepository> {
    url_repo: R,
}

impl<R: UrlRepository> UrlService<R> {
    pub fn new(url_repository: R) -> Self {
        Self {
            url_repo: url_repository,
        }
    }

    pub async fn create_short_url(&self, original_url: ValidUrl) -> Result<Url, DomainError> {
        if let Some(existing) = self.url_repo.find_by_original_url(&original_url).await? {
            return Ok(existing);
        }

        let mut short_code = ShortCode::new()?;

        let mut attempts = 0;

        while self.url_repo.exists_by_short_code(&short_code).await? {
            if attempts >= 5 {
                return Err(DomainError::ShortcodeConflict);
            }

            short_code = ShortCode::new()?;
            attempts += 1;
        }

        let url = Url::new(original_url, short_code);
        let created_url = self.url_repo.save(&url).await?;

        Ok(created_url)
    }
}
