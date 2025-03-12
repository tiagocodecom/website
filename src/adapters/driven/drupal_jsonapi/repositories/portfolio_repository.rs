use async_trait::async_trait;

use crate::adapters::driven::drupal_jsonapi::entities::{NodePortfolioResource, PortfolioNode};
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalPortfolioAdapter;
use crate::adapters::driven::drupal_jsonapi::mappers::PortfolioNodeMapper;
use crate::adapters::driven::drupal_jsonapi::services::{HttpClientService, JsonApiClientService};
use crate::application::domain::core::Result;
use crate::application::domain::portfolio::Portfolio;
use crate::application::ports::driven::ForFetchingPortfolioData;

/// Repository for fetching and transforming portfolio data from an external API.
///
/// This struct implements the `ForFetchingPortfolioData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve portfolio items and transform them into domain entities.
pub struct PortfolioRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<(dyn ExternalPortfolioAdapter<Input = PortfolioNode>)>,
}

impl PortfolioRepository {
    pub fn new(http_client: HttpClientService) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client)),
            api_adapter: Box::new(PortfolioNodeMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingPortfolioData for PortfolioRepository {
    async fn find_portfolio_by_slug(&self, slug: &str) -> Result<Portfolio> {
        let external_router = self.api_client.get_route(slug).await?;

        let resource_url = &format!(
            "/{}/{}/{}/{}",
            external_router.jsonapi().path_prefix(),
            external_router.entity().entity_type(),
            external_router.entity().bundle(),
            external_router.entity().uuid()
        );

        let external_portfolio = self
            .api_client
            .get_resource::<NodePortfolioResource>(resource_url)
            .await?;

        let portfolio = self.api_adapter.adapt(external_portfolio.data().clone())?;

        Ok(portfolio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::driven::drupal_jsonapi::services::{Config, HttpClientService};
    use crate::application::value_objects::ModerationStatus;
    use mockito::Matcher::Regex;
    use mockito::{Mock, Server};

    #[actix_rt::test]
    async fn succeeds_to_fetch_portfolio_when_slug_is_valid() {
        let mut server = Server::new_async().await;

        let router_mock = router_mock(&mut server, "/portfolio/john-doe", 200).await;
        let resource_mock = resource_mock(&mut server, "/jsonapi/node/portfolio", 200).await;
        let http_client_mock = http_client_mock(&server.url());

        let portfolio = PortfolioRepository::new(http_client_mock)
            .find_portfolio_by_slug("/portfolio/john-doe")
            .await
            .unwrap();

        assert!(router_mock.matched_async().await);
        assert!(resource_mock.matched_async().await);
        assert_eq!(portfolio.title().as_str(), "John Doe");
        assert_eq!(*portfolio.status(), ModerationStatus::Published);
        assert!(portfolio.sections().len() > 1);
    }

    #[actix_rt::test]
    async fn fails_to_fetch_portfolio_when_slug_is_invalid() {
        let mut server = Server::new_async().await;

        let router_mock = router_mock(&mut server, "/non-existing-portfolio", 404).await;
        let resource_mock = resource_mock(&mut server, "/jsonapi/node/portfolio", 200).await;
        let http_client_mock = http_client_mock(&server.url());

        let result = PortfolioRepository::new(http_client_mock)
            .find_portfolio_by_slug("/non-existing-portfolio")
            .await;

        assert!(router_mock.matched_async().await);
        assert!(!resource_mock.matched_async().await);
        assert!(result.is_err());
    }

    fn http_client_mock(url: &str) -> HttpClientService {
        HttpClientService::new(
            Config::default()
                .base_url(url.to_string())
                .basic_auth(("user", "password"))
                .build(),
        )
    }

    async fn router_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        let path = format!("/router/translate-path?path={path}");
        let body_file = &format!("tests/fixtures/http_decoupled_router_{status}.json");

        server
            .mock("GET", path.as_str())
            .with_status(status)
            .with_header("content-type", "application/json")
            .with_body_from_file(body_file)
            .create_async()
            .await
    }

    async fn resource_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        let body_file_path = &format!("tests/fixtures/http_portfolio_resource_{status}.json");

        server
            .mock("GET", Regex(path.to_string()))
            .with_status(status)
            .with_header("content-type", "application/vnd.api+json")
            .with_body_from_file(body_file_path)
            .create_async()
            .await
    }
}
