use crate::{
    application::{CreateUrlDto, UrlResponseDto},
    domain::{
        errors::DomainError,
        services::IUrlService,
        value_objects::{ShortCode, ValidUrl},
    },
};

pub struct CreateUrlCommand<'a, U: IUrlService> {
    url_service: &'a U,
}

impl<'a, U: IUrlService> CreateUrlCommand<'a, U> {
    pub fn new(url_service: &'a U) -> Self {
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
