use crate::domain::errors::DomainError;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

#[derive(serde::Serialize)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new(custom_code: Option<String>) -> Result<Self, DomainError> {
        let short_code = if let Some(custom_value) = custom_code {
            Self(custom_value)
        } else {
            let mut rng = rand::rng();
            let random_bytes: [u8; 6] = rng.random();

            // Use URL-safe Base64 encoding and take only first 8 chars
            let gen_bytes = general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);
            Self(gen_bytes[0..8].to_string())
        };

        Ok(short_code)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for ShortCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for ShortCode {
    fn into(self) -> String {
        self.0
    }
}
