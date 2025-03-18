use crate::domain::DomainError;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Result<Self, DomainError> {
        let mut rng = rand::rng();
        let random_bytes: [u8; 6] = rng.random();

        // Use URL-safe Base64 encoding and take only first 8 chars
        let gen_bytes = general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);
        let short_code = gen_bytes[0..8].to_string();

        Ok(Self(short_code))
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_inner(self) -> String {
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
