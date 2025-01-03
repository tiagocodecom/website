use crate::shared::api::models::{Error, NodePortfolio, ResolvedRoute, Resource};
use crate::shared::api::services::ApiGetService;
use crate::shared::api::utils::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;
use url::Url;

pub struct NodePortfolioService {
    http_client: Arc<HttpClient>,
}

#[async_trait(?Send)]
impl ApiGetService<Resource<NodePortfolio>> for NodePortfolioService {
    fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    async fn fetch(&self, endpoint: &str) -> Result<Resource<NodePortfolio>, Error> {
        let resolved_route = self.resolve_path(endpoint).await?;

        let jsonapi_endpoint = Url::parse(resolved_route.jsonapi().individual())
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        let node_portfolio_resource = self.get_resource(jsonapi_endpoint.path()).await?;

        Ok(node_portfolio_resource)
    }
}

impl NodePortfolioService {
    /// Resolves a path to a ResolvedRoute by making a request to the router endpoint.
    ///
    /// # Errors
    /// - `Error::NotFound` if path cannot be resolved (404)
    /// - `Error::InvalidResponse` if response cannot be parsed as JSON
    /// - `Error::DeserializationFailed` if JSON cannot be deserialized to ResolvedRoute
    pub async fn resolve_path(&self, path: &str) -> Result<ResolvedRoute, Error> {
        let response = self
            .http_client
            .get(&format!("/router/translate-path?path={path}"))
            .await?;

        if response.status().as_u16() == 404 {
            return Err(Error::NotFound(format!("Unable to resolve path {path}")));
        }

        let response = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        Ok(serde_json::from_value::<ResolvedRoute>(response)
            .map_err(|e| Error::DeserializationFailed(e.to_string()))?)
    }

    /// Fetches a NodePortfolio resource from the given path with all its relationships.
    ///
    /// # Errors
    /// - `Error::NotFound` if resource does not exist at path
    /// - `Error::InvalidResponse` if response cannot be parsed as JSON
    /// - `Error::DeserializationFailed` if JSON cannot be deserialized to NodePortfolio
    pub async fn get_resource(&self, path: &str) -> Result<Resource<NodePortfolio>, Error> {
        let response = self
            .http_client
            .get(&format!("{path}?include=content,content.document.media_document,content.image.media_image,content.items.media.media_image,content.items.items&jsonapi_include=1"))
            .await?;

        if response.status().as_u16() == 404 {
            return Err(Error::NotFound(format!("Resource not found at {path}")));
        }

        let response = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        Ok(serde_json::from_value::<Resource<NodePortfolio>>(response)
            .map_err(|e| Error::DeserializationFailed(e.to_string()))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::api::utils::ClientConfigBuilder;
    use mockito::{Matcher, Mock, Server};

    #[actix_rt::test]
    async fn gets_node_portfolio_with_relationships_successfully() {
        let mut server = Server::new_async().await;

        let http_client_mock = setup_http_client(&server.url());
        let router_mock = setup_router_mock(&mut server, "/portfolio", 200).await;
        let jsonapi_mock = setup_jsonapi_mock(&mut server, 200).await;

        let node_portfolio = NodePortfolioService::new(http_client_mock)
            .fetch("/portfolio")
            .await
            .unwrap();

        assert!(router_mock.matched_async().await); // Ensure the decoupled router was called
        assert!(jsonapi_mock.matched_async().await); // Ensure the jsonapi was called
        assert_eq!(node_portfolio.data().entity_type(), "node--portfolio"); // Ensure the data was deserialized correctly
    }

    #[actix_rt::test]
    async fn returns_error_when_path_is_not_found_by_decoupled_router() {
        let mut server = Server::new_async().await;

        let http_client_mock = setup_http_client(&server.url());
        let router_mock = setup_router_mock(&mut server, "/unknown", 404).await;
        let jsonapi_mock = setup_jsonapi_mock(&mut server, 200).await;

        let node_portfolio = NodePortfolioService::new(http_client_mock)
            .fetch("/unknown")
            .await;

        let expected_error = Error::NotFound("Unable to resolve path /unknown".into());

        assert!(router_mock.matched_async().await); // Ensure the decoupled router was called, and returned a 404
        assert!(!jsonapi_mock.matched_async().await); // Ensure the jsonapi was not called
        assert_eq!(node_portfolio.unwrap_err(), expected_error); // Ensure the error is the expected NotFound error
    }

    pub fn setup_http_client(url: &str) -> Arc<HttpClient> {
        let client_config = ClientConfigBuilder::default()
            .base_url(Some(url.to_string()))
            .build()
            .unwrap();

        Arc::new(HttpClient::new(client_config).unwrap())
    }

    pub async fn setup_router_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        let path = format!("/router/translate-path?path={path}");

        server
            .mock("GET", path.as_str())
            .with_status(status)
            .with_header("Content-Type", "application/json")
            .with_body_from_file(&format!("tests/fixtures/decoupled_router_{status}.json"))
            .create_async()
            .await
    }

    pub async fn setup_jsonapi_mock(server: &mut Server, status: usize) -> Mock {
        let path = Matcher::Regex(r"^/jsonapi/node/portfolio/".to_string());
        server
            .mock("GET", path)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("jsonapi_include".into(), "1".into()),
                Matcher::UrlEncoded("include".into(), "content,content.document.media_document,content.image.media_image,content.items.media.media_image,content.items.items".into()),
            ]))
            .with_status(status)
            .with_header("Content-Type", "application/vnd.api+json")
            .with_body_from_file(&format!("tests/fixtures/node_portfolio_{status}.json"))
            .create_async()
            .await
    }
}
