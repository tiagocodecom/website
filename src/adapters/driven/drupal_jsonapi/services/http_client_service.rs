use reqwest::{Client, Method, RequestBuilder, Response, Url};
use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde_json::Value;

/// HTTP client with optional authentication and base URL configuration.
#[derive(Default, Clone)]
pub struct HttpClientService {
    client: Client,
    config: Config,
}

impl HttpClientService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Sends a GET request and returns the JSON response.
    pub async fn get_json(&self, url: &str) -> Result<Value, String> {
        self.request(Method::GET, url)
            .await?
            .json::<Value>()
            .await
            .map_err(|e| e.to_string())
    }

    /// Sends an HTTP request with the specified method and URL.
    async fn request(&self, method: Method, url: &str) -> Result<Response, String> {
        let endpoint = self.resolve_endpoint(url)?;
        let mut request = self.client.request(method, endpoint);

        if let Some(auth) = &self.config.basic_auth {
            request = auth.apply(request);
        }

        let response = request.send().await.map_err(|e| e.to_string())?;
        if response.status().is_success() {
            return Ok(response);
        }

        Err(response.text().await.unwrap_or_else(|_| "unknown".into()))
    }

    /// Builds a valid URL by appending the base URL if needed.
    fn resolve_endpoint(&self, url: &str) -> Result<String, String> {
        if Url::parse(url).is_ok() && url.starts_with("http") {
            return Ok(url.to_string());
        }

        let full_url = format!("{}{}", self.config.base_url.as_deref().unwrap_or(""), url);
        Url::parse(&full_url)
            .map(|_| full_url)
            .map_err(|e| e.to_string())
    }
}

/// Configuration for an HTTP client, including optional `base_url` and `basic_auth`.
#[derive(Default, Clone)]
pub struct Config {
    base_url: Option<String>,
    basic_auth: Option<BasicAuth>,
}

impl Config {
    /// Sets the base API URL.
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Sets basic authentication credentials.
    pub fn basic_auth(mut self, basic_auth: (&str, &str)) -> Self {
        self.basic_auth = Some(BasicAuth::from(basic_auth));
        self
    }

    /// Returns the configured instance.
    pub fn build(self) -> Self {
        self
    }
}

/// Basic authentication credentials stored securely.
#[derive(Clone)]
pub struct BasicAuth {
    username: SecretString,
    password: SecretString,
}

impl BasicAuth {
    /// Adds basic authentication headers to an HTTP request.
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(
            self.username.expose_secret(),
            Some(self.password.expose_secret()),
        )
    }
}

impl From<(&str, &str)> for BasicAuth {
    /// Creates `BasicAuth` from a username and password.
    fn from(value: (&str, &str)) -> Self {
        Self {
            username: SecretBox::new(Box::from(value.0.to_string())),
            password: SecretBox::new(Box::from(value.1.to_string())),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockito::{Mock, Server};
//
//     #[test]
//     fn initializes_basic_authentication() {
//         let auth = BasicAuth::from(("username", "password"));
//
//         assert_eq!(auth.username.expose_secret(), "username");
//         assert_eq!(auth.password.expose_secret(), "password");
//     }
//
//     #[test]
//     fn initializes_default_client_config() {
//         let config = Config::default();
//
//         assert!(config.base_url.is_none());
//         assert!(config.basic_auth.is_none());
//     }
//
//     #[test]
//     fn initializes_custom_client_config() {
//         let config = Config::default()
//             .basic_auth(("username", "password"))
//             .base_url("https://example.com".into())
//             .build();
//
//         assert!(config.basic_auth.is_some());
//         assert_eq!(config.base_url.unwrap(), "https://example.com");
//     }
//
//     #[test]
//     fn initializes_http_client_with_default_config() {
//         let http_client = HttpClient::default();
//
//         assert!(http_client.config.base_url.is_none());
//         assert!(http_client.config.basic_auth.is_none());
//     }
//
//     #[test]
//     fn initializes_http_client_with_custom_config() {
//         let base_url = "https://proxy.custom.com";
//         let http_client_mock = setup_http_client_mock(base_url);
//
//         assert_eq!(http_client_mock.config.base_url, Some(base_url.to_string()));
//     }
//
//     #[test]
//     fn builds_an_absolute_url_without_modifying_it() {
//         let http_client = HttpClient::default();
//         let result = http_client
//             .build_url("https://example.com/api/v1/resource")
//             .unwrap();
//
//         assert_eq!(result, "https://example.com/api/v1/resource");
//     }
//
//     #[test]
//     fn builds_a_relative_url_prepending_the_base_url() {
//         let http_client_mock = setup_http_client_mock("https://example.com");
//         let result = http_client_mock.build_url("/api/v1/resource").unwrap();
//
//         assert_eq!(result, "https://example.com/api/v1/resource");
//     }
//
//     #[test]
//     fn returns_an_error_when_building_an_invalid_url() {
//         let http_client = HttpClient::default();
//         let result = http_client.build_url("..invalid..url..");
//
//         assert!(result.is_err());
//     }
//
//     #[actix_rt::test]
//     async fn makes_a_get_request_to_the_specified_url_succeeds() {
//         let mut server = Server::new_async().await;
//         let http_client_mock = setup_http_client_mock(&server.url());
//         let request_mock = setup_request_mock(&mut server, "/api/v1/resource", 200).await;
//         let response = http_client_mock.get("/api/v1/resource").await;
//
//         assert!(response.is_ok());
//         assert!(request_mock.matched_async().await);
//     }
//
//     #[actix_rt::test]
//     async fn returns_request_failed_error_when_request_fails() {
//         let mut server = Server::new_async().await;
//         let http_client_mock = setup_http_client_mock(&server.url());
//         let request_mock = setup_request_mock(&mut server, "/api/v1/resource", 500).await;
//         let response = http_client_mock.get("/api/v1/resource").await;
//
//         assert!(response.is_err());
//         assert!(request_mock.matched_async().await);
//     }
//
//     #[actix_rt::test]
//     async fn ignores_basic_auth_header_when_not_configured() {
//         let mut server = Server::new_async().await;
//         let request_mock = server
//             .mock("GET", "/example")
//             .with_status(200)
//             .with_header("Content-Type", "application/json")
//             .with_body(r#"{"message": "Hello, World!"}"#)
//             .create_async()
//             .await;
//
//         let config = Config::default().base_url(server.url()).build();
//         let http_client_mock = HttpClient::new(config);
//         let response = http_client_mock.get("/example").await;
//
//         assert!(response.is_ok()); // sends request without basic auth
//         assert!(request_mock.matched_async().await); // ensures it works
//     }
//
//     pub fn setup_http_client_mock(url: &str) -> HttpClient {
//         let config = Config::default()
//             .base_url(url.to_string())
//             .basic_auth(("test", "123456"))
//             .build();
//
//         HttpClient::new(config)
//     }
//
//     pub async fn setup_request_mock(server: &mut Server, path: &str, status: usize) -> Mock {
//         server
//             .mock("GET", path)
//             .with_status(status)
//             .with_header("Content-Type", "application/json")
//             .with_body_from_file(&format!("tests/fixtures/basic_request_{status}.json"))
//             .match_header("Authorization", "Basic dGVzdDoxMjM0NTY=")
//             .create_async()
//             .await
//     }
// }
