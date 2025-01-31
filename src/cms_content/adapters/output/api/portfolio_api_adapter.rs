use async_trait::async_trait;

use crate::cms_content::adapters::output::api::models::NodePortfolio;
use crate::cms_content::adapters::output::api::utils::CmsApiConnector;
use crate::cms_content::adapters::output::api::{CmsApi, CmsPortfolioMapper, HttpClient, Resource};
use crate::cms_content::application::ports::output::{LoadPortfolioPort, LoadPortfolioStateMapper};
use crate::cms_content::domain::error::CmsResult;
use crate::cms_content::domain::portfolio::Portfolio;

/// Adapter for loading and transforming portfolio data from an external CMS API.
///
/// This struct integrates the `CmsApiConnector` to interact with the CMS API and the
/// `LoadPortfolioStateMapper` to map the fetched resource into a `Portfolio` domain entity.
///
/// # Dependencies
/// - `cms_connector`: A connector to the CMS API to resolve paths and fetch resources.
/// - `cms_mapper`: A mapper to convert external resources into a `Portfolio` domain object.
///
/// # Methods
/// - `new`: Creates a new `PortfolioApiAdapter` with an `HttpClient`.
/// - `find_by_slug`: Resolves a path and fetches the corresponding portfolio by slug, transforming the resource into a `Portfolio`.
pub struct PortfolioApiAdapter {
    cms_connector: Box<(dyn CmsApiConnector)>,
    cms_mapper: Box<(dyn LoadPortfolioStateMapper<Input = Resource<NodePortfolio>>)>,
}

impl PortfolioApiAdapter {
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            cms_connector: Box::new(CmsApi::new(http_client)),
            cms_mapper: Box::new(CmsPortfolioMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl LoadPortfolioPort for PortfolioApiAdapter {
    async fn find_by_slug(&self, slug: &str) -> CmsResult<Portfolio> {
        let resolved_route = self.cms_connector.resolve_path(slug).await?;

        let resource = self
            .cms_connector
            .fetch_resource(resolved_route.jsonapi().individual())
            .await?;

        let portfolio = self.cms_mapper.transform(resource)?;

        Ok(portfolio)
    }
}
