use reqwest::{Client, Method, RequestBuilder, Response, Url};
use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde_json::Value;

#[derive(Default, Clone)]
pub struct HttpClientService {
    client: Client,
    config: Config,
}

#[derive(Default, Clone)]
pub struct Config {
    base_url: Option<String>,
    basic_auth: Option<BasicAuth>,
}

#[derive(Clone)]
pub struct BasicAuth {
    username: SecretString,
    password: SecretString,
}

impl HttpClientService {
    /// Creates a new instance with given configuration.
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use mockito::{Mock, Server};

    #[test]
    fn creation_succeeds_when_using_default_values() {
        let http_client = HttpClientService::default();
        assert!(http_client.config.base_url.is_none());
        assert!(http_client.config.basic_auth.is_none());
    }

    #[test]
    fn creation_succeeds_when_custom_config_is_valid() {
        let http_client_mock = http_client_mock("https://localhost.dev");
        let base_url = http_client_mock.config.base_url.unwrap();
        let basic_auth = http_client_mock.config.basic_auth.unwrap();

        assert_eq!(base_url, "https://localhost.dev");
        assert_eq!(basic_auth.username.expose_secret(), "test");
        assert_eq!(basic_auth.password.expose_secret(), "123456");
    }

    #[test]
    fn endpoint_resolver_succeeds_when_input_is_absolute_url() {
        let request_endpoint = http_client_mock("https://baseurl.dev")
            .resolve_endpoint("https://localhost.dev/api")
            .unwrap();

        assert_eq!(request_endpoint, "https://localhost.dev/api"); // base url is ignored, as it's an absolute URL
    }

    #[test]
    fn endpoint_resolver_succeeds_when_input_is_relative_url() {
        let request_endpoint = http_client_mock("https://localhost.dev")
            .resolve_endpoint("/api")
            .unwrap();

        assert_eq!(request_endpoint, "https://localhost.dev/api");
    }

    #[test]
    fn endpoint_resolver_fails_when_input_is_invalid() {
        let http_client = HttpClientService::default();
        let result = http_client.resolve_endpoint("..invalid..url..");

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn request_succeeds_when_endpoint_is_valid() {
        let mut server = Server::new_async().await;
        let http_client_mock = http_client_mock(&server.url());
        let request_mock = request_mock(&mut server, "/api/v1/resource", 200).await;
        let response = http_client_mock.get_json("/api/v1/resource").await;

        assert!(response.is_ok());
        assert!(request_mock.matched_async().await);
    }

    #[actix_rt::test]
    async fn request_fails_when_external_service_returns_error() {
        let mut server = Server::new_async().await;
        let http_client_mock = http_client_mock(&server.url());
        let request_mock = request_mock(&mut server, "/api/v1/resource", 500).await;
        let response = http_client_mock.get_json("/api/v1/resource").await;

        assert!(response.is_err());
        assert!(request_mock.matched_async().await);
    }

    #[actix_rt::test]
    async fn request_ignores_basic_auth_header_when_credentials_are_not_configured() {
        let mut server = Server::new_async().await;
        let request_mock = server
            .mock("GET", "/example")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(r#"{"message": "Hello, World!"}"#)
            .create_async()
            .await;

        let config = Config::default().base_url(server.url()).build();
        let http_client_mock = HttpClientService::new(config);
        let response = http_client_mock.get_json("/example").await;

        assert!(response.is_ok()); // sends request without basic auth
        assert!(request_mock.matched_async().await); // ensures it works
    }

    pub fn http_client_mock(url: &str) -> HttpClientService {
        let config = Config::default()
            .base_url(url.to_string())
            .basic_auth(("test", "123456"))
            .build();

        HttpClientService::new(config)
    }

    async fn request_mock(server: &mut Server, path: &str, status: usize) -> Mock {
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
