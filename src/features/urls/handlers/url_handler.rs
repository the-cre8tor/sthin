use actix_web::{
    HttpResponse, Responder,
    web::{Data, Json, Path},
};

use crate::{
    error::AppError,
    features::urls::{
        dtos::CreateUrlDto,
        repository::UrlRepository,
        service::{IUrlService, UrlService},
        value_objects::{ShortCode, ValidUrl},
    },
    infrastructure::http::ApiResponse,
};

pub struct UrlHandler;

#[derive(serde::Deserialize, Debug)]
pub struct ShortCodePath {
    pub code: String,
}

impl UrlHandler {
    pub async fn create_short_url(
        dto: Json<CreateUrlDto>,
        url_service: Data<UrlService<UrlRepository>>,
    ) -> Result<impl Responder, AppError> {
        // let _ = dto.validate()?;

        let valid_url = ValidUrl::new(dto.0.url)?;

        let url = if let Some(custom_code) = dto.0.custom_code {
            let short_code = ShortCode::new(Some(custom_code))?;

            url_service
                .create_short_url(valid_url, Some(short_code))
                .await
        } else {
            url_service.create_short_url(valid_url, None).await
        };

        match url {
            Ok(url) => Ok(ApiResponse::success(url)),
            Err(error) => Err(error.into()),
        }
    }

    pub async fn retreive_url_by_short_code(
        param: Path<String>,
        url_service: Data<UrlService<UrlRepository>>,
    ) -> Result<HttpResponse, AppError> {
        let short_code = ShortCode::new(Some(param.into_inner()))?;

        let result = url_service.get_url_by_short_code(short_code).await?;

        Ok(ApiResponse::success(result))
    }
}
