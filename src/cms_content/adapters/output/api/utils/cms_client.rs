use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};

use crate::cms_content::adapters::output::api::models::{NodePortfolio, ResolvedRoute, Resource};
use crate::cms_content::adapters::output::api::utils::HttpClient;
use crate::cms_content::domain::error::{CmsError, CmsResult};

/// API endpoint for resolving CMS paths.
const RESOLVE_PATH_ENDPOINT: &str = "/router/translate-path?path=";

/// Query parameters for fetching a CMS resource along with related content.
const RESOURCE_QUERY_PARAMS: &str = "include=content,content.document.media_document,\
content.image.media_image,content.items.media.media_image,\
content.items.items&jsonapi_include=1";

/// Defines an interface for interacting with the CMS API.
///
/// This trait provides methods for resolving paths and fetching resources.
#[async_trait(?Send)]
pub trait CmsApiConnector {
    /// Resolves a given CMS path to its corresponding route representation.
    ///
    /// # Parameters
    /// - `path`: The CMS path to be resolved.
    ///
    /// # Returns
    /// - `CmsResult<ResolvedRoute>`: The resolved route data.
    /// - `CmsError::CommunicationFailed`: If the HTTP request fails.
    /// - `CmsError::ResourceNotFound`: If the CMS path does not exist.
    async fn resolve_path(&self, path: &str) -> CmsResult<ResolvedRoute>;

    /// Fetches a CMS resource, including additional related content.
    ///
    /// # Parameters
    /// - `url`: The API endpoint for retrieving the resource.
    ///
    /// # Returns
    /// - `CmsResult<Resource<NodePortfolio>>`: The retrieved resource.
    /// - `CmsError::CommunicationFailed`: If the HTTP request fails.
    /// - `CmsError::ResourceNotFound`: If the resource is not found.
    async fn fetch_resource(&self, url: &str) -> CmsResult<Resource<NodePortfolio>>;
}

/// Provides an implementation of `CmsApiConnector`, enabling interaction with the CMS API.
pub struct CmsApi {
    http_client: HttpClient,
}

impl CmsApi {
    /// Creates a new `CmsApi` instance.
    ///
    /// # Parameters
    /// - `http_client`: The HTTP client used for API requests.
    ///
    /// # Returns
    /// - A new `CmsApi` instance.
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    /// Sends an HTTP GET request and parses the JSON response.
    ///
    /// # Parameters
    /// - `url`: The API endpoint to fetch.
    ///
    /// # Returns
    /// - `CmsResult<Value>`: The raw JSON response.
    async fn fetch_json(&self, url: &str) -> CmsResult<Value> {
        let response = self
            .http_client
            .get(url)
            .await
            .map_err(|e| CmsError::CommunicationFailed(e.to_string()))?;

        match response.status().as_u16() {
            403 => Err(CmsError::Unauthorized(url.to_string())),
            404 => Err(CmsError::ResourceNotFound(url.to_string())),
            500 => Err(CmsError::CommunicationFailed(url.to_string())),
            _ => response
                .json::<Value>()
                .await
                .map_err(|e| CmsError::Unknown(e.to_string())),
        }
    }

    /// Converts a JSON value into the expected type.
    ///
    /// # Parameters
    /// - `json_value`: The JSON data to be deserialized.
    ///
    /// # Returns
    /// - `CmsResult<T>`: The successfully parsed data.
    fn deserialize_json<T: DeserializeOwned>(json_value: Value) -> CmsResult<T> {
        from_value::<T>(json_value).map_err(|e| CmsError::Unknown(e.to_string()))
    }
}

#[async_trait(?Send)]
impl CmsApiConnector for CmsApi {
    /// Resolves a CMS path to its corresponding `ResolvedRoute`.
    async fn resolve_path(&self, path: &str) -> CmsResult<ResolvedRoute> {
        let url = format!("{RESOLVE_PATH_ENDPOINT}{path}");
        let json_value = self.fetch_json(&url).await?;
        Self::deserialize_json(json_value)
    }

    /// Fetches a resource from the CMS, appending necessary query parameters.
    async fn fetch_resource(&self, url: &str) -> CmsResult<Resource<NodePortfolio>> {
        let url = format!("{url}?{RESOURCE_QUERY_PARAMS}");
        let json_value = self.fetch_json(&url).await?;
        Self::deserialize_json(json_value)
    }
}
