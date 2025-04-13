use axum::{Json, response::IntoResponse};

#[derive(serde::Serialize)]
struct HealthCheck {
    status: String,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthCheck {
        status: "Up and running".to_string(),
    })
}
