use axum::{
    Json,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
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
    pub fn success(data: T) -> Response {
        (StatusCode::OK, Json(Self::Success { data })).into_response()
    }

    pub fn success_with_no_content() -> Response {
        StatusCode::NO_CONTENT.into_response()
    }

    pub fn fail(data: Value, status_code: StatusCode) -> Response {
        (status_code, Json(Self::Fail { data })).into_response()
    }

    pub fn error(message: impl Into<String>) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Self::Error {
                message: message.into(),
            }),
        )
            .into_response()
    }
}
