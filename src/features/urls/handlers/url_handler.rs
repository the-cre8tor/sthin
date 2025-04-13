use crate::{
    error::AppError,
    features::{
        url_stats::queue::StatsEvent,
        urls::{
            dtos::{CreateUrlDto, UpdateUrlDto},
            service::IUrlService,
            value_objects::{ShortCode, ValidUrl},
        },
    },
    infrastructure::server::{ApiResponse, AppState},
};
use axum::{
    Json,
    extract::{Path, State},
    response::Response,
};
use serde_json::Value;
use std::{sync::Arc, time::Instant};

pub struct UrlHandler;

impl UrlHandler {
    pub async fn create_short_url(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<CreateUrlDto>,
    ) -> Result<Response, AppError> {
        let valid_url = ValidUrl::new(payload.url)?;

        let service = &state.services;

        let url = if let Some(custom_code) = payload.custom_code {
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
        Path(param): Path<String>,
        State(state): State<Arc<AppState>>,
    ) -> Result<Response, AppError> {
        let short_code = ShortCode::new(Some(param))?;

        let service = &state.services;
        let queue = &state.processors;

        let result = service
            .url_service
            .get_url_by_short_code(short_code)
            .await?;

        let event = StatsEvent {
            data: result.clone(),
            timestamp: Instant::now(),
        };

        if let Err(error) = queue.stats_processor.sender.try_send(event) {
            println!("Stats channel full: {}", error)
        }

        Ok(ApiResponse::success(result))
    }

    pub async fn update_url_by_short_code(
        Path(param): Path<String>,
        State(state): State<Arc<AppState>>, // state must come second else it will be panic
        Json(payload): Json<UpdateUrlDto>,
    ) -> Result<Response, AppError> {
        let short_code = ShortCode::new(Some(param))?;
        let valid_url = ValidUrl::new(payload.url.clone())?;

        let service = &state.services;

        let response = service
            .url_service
            .update_url_by_short_code(short_code, valid_url)
            .await?;

        Ok(ApiResponse::success(response))
    }

    pub async fn delete_url_by_short_code(
        Path(param): Path<String>,
        State(state): State<Arc<AppState>>,
    ) -> Result<Response, AppError> {
        let short_code = ShortCode::new(Some(param))?;

        let service = &state.services;

        let _ = service
            .url_service
            .delete_url_by_short_code(&short_code)
            .await?;

        Ok(ApiResponse::<Value>::success_with_no_content())
    }
}
