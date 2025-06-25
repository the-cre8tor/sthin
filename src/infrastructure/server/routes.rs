use crate::features::urls::handlers::UrlHandler;
use crate::infrastructure::server::ApiResponse;
use actix_web::{
    Error, HttpRequest,
    error::{InternalError, JsonPayloadError},
    http::StatusCode,
    web::{self, JsonConfig, ServiceConfig, delete, get, patch, post, scope},
};
use serde_json::{Value, json};

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
                    .route(
                        "/{code}/stats",
                        get().to(UrlHandler::fetch_short_code_stats),
                    )
                    .route("/{code}", patch().to(UrlHandler::update_url_by_short_code))
                    .route("/{code}", delete().to(UrlHandler::delete_url_by_short_code)),
            ),
        )
        .default_service(web::route().to(|req: HttpRequest| async move {
            ApiResponse::<&str>::fail(
                json!({"error": "Route not found", "path": req.path()}),
                StatusCode::NOT_FOUND,
            )
        }));
    }
}

fn json_error_handler(error: JsonPayloadError, _req: &HttpRequest) -> Error {
    let error_message = match &error {
        JsonPayloadError::ContentType => format!("Content body must be a json object"),
        JsonPayloadError::Deserialize(json_error) => json_deserialization(json_error),
        JsonPayloadError::Payload(_) => format!("Empty or invalid payload"),
        _ => format!("Invalid JSON payload"),
    };

    let response = ApiResponse::<&str>::fail(Value::from(error_message), StatusCode::BAD_REQUEST);
    InternalError::from_response(error, response).into()
}

fn json_deserialization(json_error: &serde_json::Error) -> String {
    let detail = json_error.to_string();

    println!("detailer: {}", detail);

    if let Some(message) = extract_field_from_error(&detail) {
        format!("{}", message)
    } else {
        format!("JSON deserialization error: {}", detail)
    }
}

fn extract_field_from_error(message: &str) -> Option<String> {
    let splitter: Vec<&str> = message.split("at").collect();

    if splitter.len() >= 2 {
        let value = splitter.first();

        if let Some(string) = value {
            return Some(string.trim().to_string());
        }
    }

    None
}
