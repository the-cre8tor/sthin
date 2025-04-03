use serde::Deserialize;

use crate::error::AppError;

#[derive(Deserialize)]
pub struct UpdateUrlDto {
    #[serde(default)]
    pub url: String,
}
