use std::sync::Arc;

use super::handlers::UrlHandler;
use crate::infrastructure::server::AppState;
// use actix_web::{
//     Error, HttpRequest,
//     error::{InternalError, JsonPayloadError},
//     http::StatusCode,
// };
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub struct Routes;

impl Routes {
    pub fn configure_routes() -> Router<Arc<AppState>> {
        Router::new().nest(
            "/api",
            Router::new().nest(
                "/shorten",
                Router::new()
                    .route("/", post(UrlHandler::create_short_url))
                    .route("/{code}", get(UrlHandler::retreive_url_by_short_code))
                    .route("/{code}", put(UrlHandler::update_url_by_short_code))
                    .route("/{code}", delete(UrlHandler::delete_url_by_short_code)),
            ),
        )
    }
}

// fn json_error_handler(error: JsonPayloadError, _req: &HttpRequest) -> Error {
//     let error_message = match &error {
//         JsonPayloadError::ContentType => "Content type must be application/json",
//         JsonPayloadError::Deserialize(_json_err) => "Invalid JSON format",
//         JsonPayloadError::Payload(_) => "Empty or invalid payload",
//         _ => "Invalid JSON payload",
//     };

//     let response = ApiResponse::<&str>::fail(error_message.into(), StatusCode::BAD_REQUEST);
//     InternalError::from_response(error, response).into()
// }
