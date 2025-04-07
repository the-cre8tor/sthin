use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde_json::Value;

use crate::{
    error::AppError,
    features::urls::{
        dtos::{CreateUrlDto, UpdateUrlDto},
        service::IUrlService,
        value_objects::{ShortCode, ValidUrl},
    },
    infrastructure::{http::ApiResponse, server::AppServices},
};

pub struct UrlHandler;

impl UrlHandler {
    pub async fn create_short_url(
        payload: Json<CreateUrlDto>,
        service: Data<AppServices>,
    ) -> Result<HttpResponse, AppError> {
        let valid_url = ValidUrl::new(payload.0.url)?;

        let url = if let Some(custom_code) = payload.0.custom_code {
            let short_code = ShortCode::new(Some(custom_code))?;

            service
                .url_service
                .create_short_url(valid_url, Some(short_code))
                .await
        } else {
            service.url_service.create_short_url(valid_url, None).await
        };

        match url {
            Ok(url) => Ok(ApiResponse::success(url)),
            Err(error) => Err(error.into()),
        }
    }

    pub async fn retreive_url_by_short_code(
        param: Path<String>,
        service: Data<AppServices>,
    ) -> Result<HttpResponse, AppError> {
        let short_code = ShortCode::new(Some(param.into_inner()))?;

        let result = service
            .url_service
            .get_url_by_short_code(short_code)
            .await?;

        Ok(ApiResponse::success(result))
    }

    pub async fn update_url_by_short_code(
        param: Path<String>,
        payload: Json<UpdateUrlDto>,
        service: Data<AppServices>,
    ) -> Result<HttpResponse, AppError> {
        let short_code = ShortCode::new(Some(param.into_inner()))?;
        let valid_url = ValidUrl::new(payload.url.clone())?;

        let response = service
            .url_service
            .update_url_by_short_code(short_code, valid_url)
            .await?;

        Ok(ApiResponse::success(response))
    }

    pub async fn delete_url_by_short_code(
        param: Path<String>,
        service: Data<AppServices>,
    ) -> Result<HttpResponse, AppError> {
        let short_code = ShortCode::new(Some(param.into_inner()))?;

        let _ = service
            .url_service
            .delete_url_by_short_code(&short_code)
            .await?;

        Ok(ApiResponse::<Value>::success_with_no_content())
    }
}
