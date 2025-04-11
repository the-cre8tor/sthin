use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

use crate::features::urls::errors::UrlError;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new(custom_code: Option<String>) -> Result<Self, UrlError> {
        let short_code = if let Some(custom_value) = custom_code {
            let min = 3;
            let max = 8;

            if custom_value.len() < min || custom_value.len() > max {
                return Err(UrlError::InvalidShortCode(format!(
                    "Custom code can not be less than {min} or greater than {max}",
                )));
            }

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

#[cfg(test)]
mod test {
    use super::ShortCode;

    #[test]
    fn generate_short_code_when_one_is_not_supplied() {
        let short_code = ShortCode::new(None);
        assert!(short_code.is_ok());
    }

    #[test]
    fn convert_custom_code_into_short_code() {
        // Given
        let custom_code = "brilliant_coder";

        // When
        let result = ShortCode::new(Some(custom_code.into()));

        // Then
        assert!(result.is_ok());
        let short_code = result.unwrap();
        assert_eq!(short_code.as_str(), custom_code);
    }
}
