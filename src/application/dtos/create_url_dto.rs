use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUrlDto {
    #[validate(url(message = "Must be a valid URL"))]
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_code: Option<String>,
}
