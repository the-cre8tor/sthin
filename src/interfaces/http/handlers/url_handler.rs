use actix_web::{
    HttpResponse, Responder,
    web::{Data, Json},
};
use serde_json::json;

use crate::{
    application::{CreateUrlCommand, CreateUrlDto},
    domain::{errors::DomainError, services::UrlService},
    infrastructure::database::repositories::url_repository::UrlRepository,
};

pub struct UrlHandler;

impl UrlHandler {
    pub async fn create_short_url(
        dto: Json<CreateUrlDto>,
        url_service: Data<UrlService<UrlRepository>>,
    ) -> impl Responder {
        let command = CreateUrlCommand::new(url_service.get_ref());
        let invoke = command.execute(dto.0).await;

        match invoke {
            Ok(url) => HttpResponse::Created().json(url),
            Err(error) => match error {
                DomainError::InvalidUrl => {
                    HttpResponse::NotFound().json(json!({"error": "Failed to save valid email"}))
                }
                _ => HttpResponse::InternalServerError().json(
                    json!({ "error": "Failed to retrieve URL", "message": error.to_string() }),
                ),
            },
        }
    }
}
