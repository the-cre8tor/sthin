use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ApiResponse<T> {
    Success { data: T },
    Fail { data: Value },
    Error { message: String },
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self::Success { data })
    }

    pub fn success_with_no_content() -> HttpResponse {
        HttpResponse::NoContent().finish()
    }

    pub fn fail(data: Value, status_code: StatusCode) -> HttpResponse {
        HttpResponse::build(status_code).json(Self::Fail { data })
    }

    pub fn error(message: impl Into<String>) -> HttpResponse {
        HttpResponse::InternalServerError().json(Self::Error {
            message: message.into(),
        })
    }
}
