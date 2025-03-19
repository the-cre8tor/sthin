use crate::domain::errors::DomainError;
use url::Url as ParseUrl;

pub struct ValidUrl(String);

impl ValidUrl {
    pub fn new(url: String) -> Result<Self, DomainError> {
        match ParseUrl::parse(&url) {
            Ok(parsed) if parsed.scheme() == "http" || parsed.scheme() == "https" => {
                if url.len() > 2048 {
                    return Err(DomainError::UrlTooLong(2048));
                }

                Ok(Self(url))
            }
            Ok(_) => Err(DomainError::InvalidUrl),
            Err(_) => Err(DomainError::InvalidUrl),
        }
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
