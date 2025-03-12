use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::application::domain::core::AppError;

lazy_static! {
    static ref VALID_URL_REGEX: Regex = Regex::new(
        r"^(mailto:[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}|https?://[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?)*(:\d+)?(/[a-zA-Z0-9%_~+.&=@-]*)*(\?[a-zA-Z0-9&%=+_.,-]*)?(#[a-zA-Z0-9_-]+)?|(/[^?#]+)(\?[^#]*)?(#.*)?|(\?[^#]+)|(#.+)|/)$"
    ).unwrap();
}

/// Validates a URL string against the defined regular expression
fn is_valid_url(url: &str) -> bool {
    VALID_URL_REGEX.is_match(url)
}

/// A value object that represents a URL.
///
/// This type ensures the URL string follows a valid format for both absolute
/// (http/https) and relative URLs. It provides functionality to check if a URL
/// is absolute or relative.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url(String);

impl Url {
    /// Creates a new URL instance after validating the input string
    pub fn new(url: String) -> Result<Self, AppError> {
        match is_valid_url(&url) {
            true => Ok(Self(url)),
            false => Err(AppError::InvalidValue(type_name::<Self>(), url)),
        }
    }

    /// Checks if the URL is absolute
    pub fn is_absolute(&self) -> bool {
        self.0.starts_with("http://") || self.0.starts_with("https://")
    }
}

impl TryFrom<String> for Url {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Url {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl Deref for Url {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("https://example.com")]
    #[case("http://example.com")]
    #[case("https://sub.example.com")]
    #[case("https://local-sub.example.com")]
    #[case("https://example.com/path/to/resource")]
    #[case("https://example.com?query=value&other=param")]
    #[case("https://example.com/path?query=value#fragment")]
    #[case("https://example.com:8080")]
    #[case("https://example.com/api/v1/resource")]
    #[case("https://example.com/api/v1/resource.png")]
    #[case("https://www.example.com/@example")]
    #[case("https://www.example.com/@example")]
    fn creation_succeeds_when_input_is_absolute_url(#[case] input: &str) {
        let url = Url::try_from(input).unwrap();
        assert!(url.is_absolute());
    }

    #[rstest]
    #[case("/")]
    #[case("/example-path")]
    #[case("/nested/path/to/resource")]
    #[case("/path?query=value")]
    #[case("/path#fragment")]
    #[case("?query=params")]
    #[case("#fragment")]
    #[case("mailto:example@example.com")]
    fn creation_succeeds_when_input_is_relative_url(#[case] input: &str) {
        let url = Url::try_from(input).unwrap();
        assert!(!url.is_absolute());
    }

    #[rstest]
    #[case("")]
    #[case("https://")]
    #[case("https://-example.com")]
    #[case("https://example-.com")]
    #[case("https://example.com/<>?")]
    #[case("https://example.com:port")]
    #[case("https:// example.com")]
    #[case("ftp://example.com")]
    fn creation_fails_when_invalid_input(#[case] input: &str) {
        assert!(Url::try_from(input).is_err());
    }
}
