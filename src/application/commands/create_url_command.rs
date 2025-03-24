use crate::{
    application::{CreateUrlDto, UrlResponseDto},
    domain::{
        errors::DomainError,
        services::IUrlService,
        value_objects::{ShortCode, ValidUrl},
    },
};

pub struct CreateUrlCommand<U: IUrlService + Send + Sync> {
    url_service: U,
}

impl<U: IUrlService> CreateUrlCommand<U> {
    pub fn new(url_service: U) -> Self {
        Self { url_service }
    }

    pub async fn execute(&self, dto: CreateUrlDto) -> Result<UrlResponseDto, DomainError> {
        let valid_url = ValidUrl::new(dto.url)?;

        let url = if let Some(custom_code) = dto.custom_code {
            let short_code = ShortCode::new(Some(custom_code))?;

            self.url_service
                .create_short_url(valid_url, Some(short_code))
                .await?
        } else {
            self.url_service.create_short_url(valid_url, None).await?
        };

        Ok(UrlResponseDto::from(url))
    }
}
