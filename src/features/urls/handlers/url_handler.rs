use actix_web::{
    HttpResponse, Responder,
    web::{Data, Json},
};
use anyhow::Context;
use serde_json::json;
use validator::Validate;

use crate::{
    error::AppError,
    features::urls::{
        dtos::CreateUrlDto,
        errors::DomainError,
        repository::UrlRepository,
        service::{IUrlService, UrlService},
        value_objects::{ShortCode, ValidUrl},
    },
};

pub struct UrlHandler;

impl UrlHandler {
    pub async fn create_short_url(
        dto: Json<CreateUrlDto>,
        url_service: Data<UrlService<UrlRepository>>,
    ) -> Result<impl Responder, AppError> {
        let _ = dto.validate()?;

        let valid_url = ValidUrl::new(dto.url.clone()).context("Failed to validate url")?;

        let url = if let Some(custom_code) = dto.custom_code.clone() {
            let short_code =
                ShortCode::new(Some(custom_code)).context("Failed to validate shortcode")?;

            url_service
                .create_short_url(valid_url, Some(short_code))
                .await
        } else {
            url_service.create_short_url(valid_url, None).await
        };

        match url {
            Ok(url) => Ok(HttpResponse::Created().json(url)),
            Err(error) => match error {
                DomainError::InvalidUrl => Err(AppError::NotFound),
                _ => Ok(HttpResponse::InternalServerError().json(
                    json!({ "error": "Failed to retrieve URL", "message": error.to_string() }),
                )),
            },
        }
    }
}
