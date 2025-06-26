use std::{borrow::Cow, str::FromStr, time::Instant};

use actix_web::{
    HttpRequest, HttpResponse,
    http::header::{HeaderValue, ToStrError, USER_AGENT},
    web::{Data, Json, Path},
};
use serde_json::Value;

use actix_web::http::header;

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
    infrastructure::server::{ApiResponse, AppServices, QueueProcessor},
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

    /// Retrieves the original URL associated with a short code.
    ///
    /// # Arguments
    /// * `param` - Short code from the URL path
    /// * `service` - Application services container
    ///
    /// # Returns
    /// Redirect to the original short code URL
    pub async fn retreive_url_by_short_code(
        param: Path<String>,
        service: Data<AppServices>,
        queue: Data<QueueProcessor>,
        req: HttpRequest,
    ) -> Result<HttpResponse, AppError> {
        let connection_info = req.connection_info();
        let ip = connection_info.realip_remote_addr().unwrap_or("unknown");
        let user_agent = UrlHandler::user_agent(&req);

        let short_code = ShortCode::new(Some(param.into_inner()))?;

        let result = service
            .url_service
            .get_url_by_short_code(short_code)
            .await?;

        let event = StatsEvent {
            data: result.clone(),
            timestamp: Instant::now(),
        };

        if let Err(error) = queue.stats_processor.sender.try_send(event) {
            println!("Stats channel full: {}", error) // replace this with trace
        }

        Ok(ApiResponse::<&str>::redirect(result.original_url.as_str()))
    }

    fn user_agent(req: &HttpRequest) -> String {
        req.headers()
            .get(header::USER_AGENT)
            .map(|agent| {
                agent.to_str().map(Cow::Borrowed).unwrap_or_else(|_| {
                    Cow::Owned(String::from_utf8_lossy(agent.as_bytes()).into_owned())
                })
            })
            .unwrap_or_else(|| Cow::Borrowed("unknown"))
            .to_string()
    }

    pub async fn fetch_short_code_stats(
        param: Path<String>,
        service: Data<AppServices>,
    ) -> Result<HttpResponse, AppError> {
        Ok(ApiResponse::success({}))
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
