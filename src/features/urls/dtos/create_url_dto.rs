use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUrlDto {
    pub url: String,
    pub custom_code: Option<String>,
}
