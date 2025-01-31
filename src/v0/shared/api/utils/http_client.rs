use crate::shared::api::models::Error;
use derive_builder::Builder;
use reqwest::{Client, Response};
use reqwest::{Method, Url};
use std::sync::Arc;

#[derive(Clone)]
pub struct HttpClient {
    client: Arc<Client>,
    config: ClientConfig,
}

impl HttpClient {
    pub fn new(config: ClientConfig) -> Result<Self, String> {
        let client = Arc::new(Client::new());

        Ok(Self { config, client })
    }

    /// Makes a GET request to the specified URL. The URL will be processed through
    /// build_url() to handle relative/absolute URLs appropriately.
    ///
    /// # Errors
    /// Returns `Error::RequestFailed` if the request fails or returns an error status code
    /// Returns `Error::InvalidRequest` if the URL is not valid
    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.request(Method::GET, url).await
    }

    /// Makes an HTTP request with the specified method and URL. The URL will be processed through
    /// build_url() to handle relative/absolute URLs appropriately.
    ///
    /// # Errors
    /// Returns `Error::RequestFailed` if the request fails or returns an error status code
    /// Returns `Error::Unknown` if the request fails for an unknown reason
    pub async fn request(&self, method: Method, url: &str) -> Result<Response, Error> {
        let mut request = self.client.request(method, self.build_url(url)?);

        if let Some(basic_auth) = &self.config.basic_auth.clone() {
            request = request.basic_auth(&basic_auth.username, Some(&basic_auth.password));
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        if response.status().is_server_error() {
            return Err(Error::RequestFailed(
                response.status().as_u16(),
                response.text().await.unwrap(),
            ));
        }

        Ok(response)
    }

    /// Builds a complete URL by either returning the input URL if it's already absolute,
    /// or prepending the base URL if configured. Returns an error if the resulting URL is invalid.
    ///
    /// # Errors
    /// Returns `Error::InvalidRequest` if the resulting URL is not a valid URL format
    fn build_url(&self, url: &str) -> Result<String, Error> {
        if let Ok(parsed) = Url::parse(url) {
            if parsed.scheme() == "http" || parsed.scheme() == "https" {
                return Ok(url.to_string());
            }
        }

        let full_url = match &self.config.base_url {
            Some(base) => format!("{base}{url}"),
            None => url.to_string(),
        };

        Url::parse(&full_url)
            .map(|_| full_url)
            .map_err(|e| Error::InvalidUrl(e.to_string()))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        let config = ClientConfigBuilder::default()
            .build()
            .expect("Failed to build default config");

        Self::new(config).expect("Failed to create default HttpClient")
    }
}

#[derive(Builder, Clone, Debug)]
pub struct ClientConfig {
    #[builder(default)]
    base_url: Option<String>,
    #[builder(default)]
    basic_auth: Option<BasicAuth>,
}

#[derive(Clone, Debug)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl From<(&str, &str)> for BasicAuth {
    fn from(value: (&str, &str)) -> Self {
        Self {
            username: value.0.to_string(),
            password: value.1.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Mock, Server};

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
        let url = "https://example.com/api/v1/resource";
        let result = http_client.build_url(url).unwrap();

        assert_eq!(result, url);
    }

    #[test]
    fn builds_a_relative_url_prepending_the_base_url() {
        let base_url = "https://example.com";
        let path = "/api/v1/resource";

        let http_client_mock = setup_http_client_mock(base_url);
        let result = http_client_mock.build_url(path).unwrap();

        assert_eq!(result, format!("{base_url}{path}"));
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

    #[test]
    pub fn instances_basic_auth_from_touple_strings() {
        let username = "test";
        let password = "123456";

        let basic_auth: BasicAuth = (username, password).into();

        assert_eq!(basic_auth.username, username);
        assert_eq!(basic_auth.password, password);
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

        let config = ClientConfigBuilder::default()
            .base_url(Some(server.url()))
            .basic_auth(None)
            .build()
            .unwrap();
        let http_client_mock = HttpClient::new(config).unwrap();
        let response = http_client_mock.get("/example").await;

        assert!(response.is_ok()); // sends request without basic auth
        assert!(request_mock.matched_async().await); // ensures it works
    }

    pub fn setup_http_client_mock(url: &str) -> HttpClient {
        let client_config = ClientConfigBuilder::default()
            .base_url(Some(url.to_string()))
            .basic_auth(Some(("test", "123456").into()))
            .build()
            .unwrap();

        HttpClient::new(client_config).unwrap()
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
