use reqwest::{Client, Method, RequestBuilder, Response, Url};
use secrecy::{CloneableSecret, ExposeSecret, SecretBox, SecretString};

/// `Config` struct represents a configuration object that holds optional settings for
/// `base_url` (API endpoint) and `basic_auth` (authentication credentials).
///
/// This struct allows chaining method calls to set values and build a final configuration.
#[derive(Default, Clone)]
pub struct Config {
    base_url: Option<String>,
    basic_auth: Option<BasicAuth>,
}

impl Config {
    /// Sets the `base_url` for the configuration.
    ///
    /// # Arguments
    /// * `base_url` - A `String` representing the base URL of the API endpoint.
    ///
    /// # Returns
    /// A new instance of `Config` with the `base_url` field set.
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Sets the `basic_auth` credentials for the configuration.
    ///
    /// # Arguments
    /// * `basic_auth` - A `BasicAuth` instance containing authentication credentials.
    ///
    /// # Returns
    /// A new instance of `Config` with the `basic_auth` field set.
    pub fn basic_auth(mut self, basic_auth: (&str, &str)) -> Self {
        self.basic_auth = Some(BasicAuth::from(basic_auth));
        self
    }

    /// Builds and returns the final `Config` instance with the current values of the fields.
    ///
    /// # Returns
    /// The `Config` instance, potentially with `base_url` and/or `basic_auth` set.
    pub fn build(self) -> Self {
        Self {
            base_url: self.base_url,
            basic_auth: self.basic_auth,
        }
    }
}

/// Stores basic authentication credentials securely.
///
/// This struct uses `SecretString` to ensure that sensitive
/// data (username and password) is stored securely and not accidentally logged or leaked.
#[derive(Clone)]
pub struct BasicAuth {
    username: SecretString,
    password: SecretString,
}

impl BasicAuth {
    /// Applies basic authentication headers to an HTTP request.
    ///
    /// # Parameters
    /// - `request`: The `RequestBuilder` to which authentication will be applied.
    ///
    /// # Returns
    /// - A modified `RequestBuilder` with the `Authorization` header set.
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(
            self.username.expose_secret(),
            Some(self.password.expose_secret()),
        )
    }
}

impl From<(&str, &str)> for BasicAuth {
    fn from(value: (&str, &str)) -> Self {
        Self {
            username: SecretBox::new(Box::from(value.0.to_string())),
            password: SecretBox::new(Box::from(value.1.to_string())),
        }
    }
}

/// A lightweight HTTP client for making requests.
///
/// This struct provides a wrapper around `reqwest::Client` with support for:
/// - Base URL configuration.
/// - Basic authentication.
/// - Utility functions for making HTTP requests.
#[derive(Default, Clone)]
pub struct HttpClient {
    client: Client,
    config: Config,
}

impl HttpClient {
    /// Creates a new `HttpClient` instance.
    ///
    /// # Parameters
    /// - `config`: Configuration settings for the client, including base URL and authentication.
    ///
    /// # Returns
    /// A new instance of `HttpClient`.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Sends a `GET` request to the specified URL.
    ///
    /// # Parameters
    /// - `url`: The target URL or path (relative if base URL is set).
    ///
    /// # Returns
    /// - `Ok(Response)`: The response from the server.
    /// - `Err(String)`: An error message if the request fails.
    pub async fn get(&self, url: &str) -> Result<Response, String> {
        self.request(Method::GET, url).await
    }

    /// Sends an HTTP request with the specified method and URL.
    ///
    /// This method automatically applies basic authentication if configured.
    ///
    /// # Parameters
    /// - `method`: The HTTP method (e.g., `GET`, `POST`).
    /// - `url`: The target URL or relative path.
    ///
    /// # Returns
    /// - `Ok(Response)`: The response from the server.
    /// - `Err(String)`: An error message if the request fails
    async fn request(&self, method: Method, url: &str) -> Result<Response, String> {
        let url = self.build_url(url)?;
        let mut request = self.client.request(method, url);

        if let Some(auth) = &self.config.basic_auth {
            request = auth.apply(request);
        }

        let response = request.send().await.map_err(|e| e.to_string())?;

        if response.status().is_server_error() {
            return Err(response.text().await.unwrap());
        }

        Ok(response)
    }

    /// Builds a fully qualified URL from a given path or returns the path if it's already an absolute URL.
    ///
    /// If the provided `url` is an absolute URL (starting with "http"), it is returned as-is.
    /// Otherwise, the `url` is concatenated with the configured `base_url` (if available) to form a complete URL.
    ///
    /// # Parameters
    /// - `url`: A relative or absolute URL path.
    ///
    /// # Returns
    /// - `Ok(String)`: A fully qualified URL if the `url` is valid.
    /// - `Err(String)`: An error message if the resulting URL is invalid.
    fn build_url(&self, url: &str) -> Result<String, String> {
        if Url::parse(url).is_ok() && url.starts_with("http") {
            return Ok(url.to_string());
        }

        let full_url = format!("{}{}", self.config.base_url.as_deref().unwrap_or(""), url);

        Url::parse(&full_url)
            .map(|_| full_url)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Mock, Server};

    #[test]
    fn initializes_basic_authentication() {
        let auth = BasicAuth::from(("username", "password"));

        assert_eq!(auth.username.expose_secret(), "username");
        assert_eq!(auth.password.expose_secret(), "password");
    }

    #[test]
    fn initializes_default_client_config() {
        let config = Config::default();

        assert!(config.base_url.is_none());
        assert!(config.basic_auth.is_none());
    }

    #[test]
    fn initializes_custom_client_config() {
        let config = Config::default()
            .basic_auth(("username", "password"))
            .base_url("https://example.com".into())
            .build();

        assert!(config.basic_auth.is_some());
        assert_eq!(config.base_url.unwrap(), "https://example.com");
    }

    #[test]
    fn initializes_http_client_with_default_config() {
        let http_client = HttpClient::default();

        assert!(http_client.config.base_url.is_none());
        assert!(http_client.config.basic_auth.is_none());
    }

    #[test]
    fn initializes_http_client_with_custom_config() {
        let base_url = "https://proxy.custom.com";
        let http_client_mock = setup_http_client_mock(base_url);

        assert_eq!(http_client_mock.config.base_url, Some(base_url.to_string()));
    }

    #[test]
    fn builds_an_absolute_url_without_modifying_it() {
        let http_client = HttpClient::default();
        let result = http_client
            .build_url("https://example.com/api/v1/resource")
            .unwrap();

        assert_eq!(result, "https://example.com/api/v1/resource");
    }

    #[test]
    fn builds_a_relative_url_prepending_the_base_url() {
        let http_client_mock = setup_http_client_mock("https://example.com");
        let result = http_client_mock.build_url("/api/v1/resource").unwrap();

        assert_eq!(result, "https://example.com/api/v1/resource");
    }

    #[test]
    fn returns_an_error_when_building_an_invalid_url() {
        let http_client = HttpClient::default();
        let result = http_client.build_url("..invalid..url..");

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn makes_a_get_request_to_the_specified_url_succeeds() {
        let mut server = Server::new_async().await;
        let http_client_mock = setup_http_client_mock(&server.url());
        let request_mock = setup_request_mock(&mut server, "/api/v1/resource", 200).await;
        let response = http_client_mock.get("/api/v1/resource").await;

        assert!(response.is_ok());
        assert!(request_mock.matched_async().await);
    }

    #[actix_rt::test]
    async fn returns_request_failed_error_when_request_fails() {
        let mut server = Server::new_async().await;
        let http_client_mock = setup_http_client_mock(&server.url());
        let request_mock = setup_request_mock(&mut server, "/api/v1/resource", 500).await;
        let response = http_client_mock.get("/api/v1/resource").await;

        assert!(response.is_err());
        assert!(request_mock.matched_async().await);
    }

    #[actix_rt::test]
    async fn ignores_basic_auth_header_when_not_configured() {
        let mut server = Server::new_async().await;
        let request_mock = server
            .mock("GET", "/example")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(r#"{"message": "Hello, World!"}"#)
            .create_async()
            .await;

        let config = Config::default().base_url(server.url()).build();
        let http_client_mock = HttpClient::new(config);
        let response = http_client_mock.get("/example").await;

        assert!(response.is_ok()); // sends request without basic auth
        assert!(request_mock.matched_async().await); // ensures it works
    }

    pub fn setup_http_client_mock(url: &str) -> HttpClient {
        let config = Config::default()
            .base_url(url.to_string())
            .basic_auth(("test", "123456"))
            .build();

        HttpClient::new(config)
    }

    pub async fn setup_request_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        server
            .mock("GET", path)
            .with_status(status)
            .with_header("Content-Type", "application/json")
            .with_body_from_file(&format!("tests/fixtures/basic_request_{status}.json"))
            .match_header("Authorization", "Basic dGVzdDoxMjM0NTY=")
            .create_async()
            .await
    }
}
