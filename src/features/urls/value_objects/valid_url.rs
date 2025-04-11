use lazy_static::lazy_static;
use url::Url as ParseUrl;

use crate::features::urls::errors::UrlError;

lazy_static! {
    static ref VALID_TLDS: Vec<&'static str> = vec![
        "com", "org", "net", "edu", "gov", "mil", "int", "io", "dev", "app", "co", "me", "us",
        "uk", "de",
    ];
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ValidUrl(String);

impl ValidUrl {
    pub fn new(url: String) -> Result<Self, UrlError> {
        match ParseUrl::parse(&url) {
            Ok(parsed) if parsed.scheme() == "http" || parsed.scheme() == "https" => {
                if url.len() > 2048 {
                    return Err(UrlError::UrlTooLong(2048));
                }

                if parsed.host_str().is_none() {
                    return Err(UrlError::InvalidUrl("Missing host name".to_string()));
                }

                match parsed.host_str() {
                    Some(domain) => {
                        Self::validate_domain(domain)?;
                        Self::validate_tld(domain)?;
                    }
                    _ => {}
                }

                Ok(Self(url))
            }
            Ok(_) => Err(UrlError::InvalidUrl(
                "The URL must start with either http/https".to_string(),
            )),
            Err(msg) => {
                println!("Parsed error: {}", msg);
                Err(UrlError::InvalidUrl(msg.to_string()))
            }
        }
    }

    fn validate_domain(domain: &str) -> Result<(), UrlError> {
        if !domain.contains(".") {
            return Err(UrlError::InvalidUrl(
                "Domain must contain at least one dot".into(),
            ));
        }

        let parts: Vec<&str> = domain.split('.').collect();

        if parts.len() < 2 {
            return Err(UrlError::InvalidUrl(
                "Domain must have at least two parts".into(),
            ));
        }

        for part in parts {
            if part.is_empty() {
                return Err(UrlError::InvalidUrl("Domain parts cannot be empty".into()));
            }

            if part.starts_with('-') || part.ends_with('-') {
                return Err(UrlError::InvalidUrl(
                    "Domain parts cannot start or end with hyphen".into(),
                ));
            }

            if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                return Err(UrlError::InvalidUrl(
                    "Domain parts can only contain letters, numbers, and hyphens".into(),
                ));
            }
        }

        Ok(())
    }

    fn validate_tld(domain: &str) -> Result<(), UrlError> {
        let tld = domain
            .split('.')
            .last()
            .ok_or_else(|| UrlError::InvalidUrl("Missing TLD".into()))?;

        if !VALID_TLDS.contains(&tld) {
            return Err(UrlError::InvalidUrl(format!("Invalid TLD: {}", tld)));
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for ValidUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for ValidUrl {
    fn into(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_valid_urls() {
        let valid_urls = vec![
            "https://www.google.com",
            "http://example.org",
            "https://sub.domain.co.uk",
            "http://test.io",
        ];

        for url in valid_urls {
            assert!(
                ValidUrl::new(url.to_string()).is_ok(),
                "URL should be valid: {}",
                url
            );
        }
    }

    #[test]
    fn test_invalid_urls() {
        let invalid_urls = vec![
            "http://www.google",
            "http://google",
            "http://google.",
            "http://.com",
            "http://google..com",
            "http://google-.com",
            "http://goo gle.com",
            "http://göogle.com",
        ];

        for url in invalid_urls {
            assert!(
                ValidUrl::new(url.into()).is_err(),
                "URL should be invalid: {}",
                url
            );
        }
    }

    #[test]
    fn test_tld_validation() {
        // Should fail with invalid TLD
        let result = ValidUrl::new("http://example.invalid".into());
        let _message = String::from_str("Invalid TLD: {}");

        assert!(matches!(result, Err(UrlError::InvalidUrl(_message))));

        // Should pass with valid TLD
        let result = ValidUrl::new("http://example.com".into());
        assert!(result.is_ok());
    }
}
