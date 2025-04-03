use super::handlers::UrlHandler;
use crate::infrastructure::http::ApiResponse;
use actix_web::{
    Error, HttpRequest,
    error::{InternalError, JsonPayloadError},
    http::StatusCode,
    web::{JsonConfig, ServiceConfig, get, post, scope},
};
use serde_json::Value;

pub struct Routes;

impl Routes {
    pub fn configure_routes(cfg: &mut ServiceConfig) {
        cfg.app_data(
            JsonConfig::default()
                .limit(4096) // limit payload size
                .error_handler(json_error_handler),
        )
        .service(
            scope("/api").service(
                scope("/shorten")
                    .route("", post().to(UrlHandler::create_short_url))
                    .route("/{code}", get().to(UrlHandler::retreive_url_by_short_code))
                    .route("/{code}", post().to(UrlHandler::update_url_by_short_code)),
            ),
        );
    }
}

fn json_error_handler(error: JsonPayloadError, _req: &HttpRequest) -> Error {
    let error_message = match &error {
        JsonPayloadError::ContentType => "Content type must be application/json",
        JsonPayloadError::Deserialize(_json_err) => "Invalid JSON format",
        JsonPayloadError::Payload(_) => "Empty or invalid payload",
        _ => "Invalid JSON payload",
    };

    InternalError::from_response(
        error,
        ApiResponse::<Value>::fail(Value::String(error_message.into()), StatusCode::BAD_REQUEST),
    )
    .into()
}
