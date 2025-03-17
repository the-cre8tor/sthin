use actix_web::{Responder, Result, web::Json};

#[derive(serde::Serialize)]
struct HealthCheck {
    status: String,
}

pub async fn health_check() -> Result<impl Responder> {
    Ok(Json(HealthCheck {
        status: "Up and running".to_string(),
    }))
}
