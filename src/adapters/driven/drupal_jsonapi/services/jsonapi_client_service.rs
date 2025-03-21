use crate::adapters::driven::drupal_jsonapi::entities::Navigation;
use crate::adapters::driven::drupal_jsonapi::entities::ResolvedRoute;
use crate::adapters::driven::drupal_jsonapi::services::HttpClientService;
use crate::application::domain::core::{AppError, Result};
use serde::de::DeserializeOwned;
use serde_json::from_value;
use std::any::type_name;
use std::fmt::Debug;

/// API endpoint for resolving CMS paths.
const TRANSLATE_PATH_ENDPOINT: &str = "/router/translate-path?path=";

/// API endpoint for fetching menu items.
const MENU_TREE_ENDPOINT: &str = "/api/menu_items/";

/// Query parameters for fetching a CMS resource along with related content.
const PORTFOLIO_QUERY: &str = "\
    include=content,content.document.media_document,content.image.media_image,content.items.media.media_image,content.items.items\
    &jsonapi_include=1";

const ARTICLES_QUERY: &str = "\
    include=thumbnail.media_image,tags\
    &filter[status]=1&sort=created,title\
    &filter[promoted]=1\
    &page[limit]=2\
    &sort[sort-created][path]=created&sort[sort-created][direction]=desc\
    &jsonapi_include=1";

pub struct JsonApiClientService {
    http_client: HttpClientService,
}

impl JsonApiClientService {
    pub fn new(http_client: HttpClientService) -> Self {
        Self { http_client }
    }

    pub async fn fetch_json(&self, url: &str) -> Result<serde_json::Value> {
        self.http_client
            .get_json(url)
            .await
            .map_err(|e| AppError::External(type_name::<Self>(), e.to_string()))
    }

    pub async fn get_route(&self, path: &str) -> Result<ResolvedRoute> {
        let url = format!("{TRANSLATE_PATH_ENDPOINT}{path}");
        let json = self.fetch_json(&url).await?;

        from_value(json.clone())
            .map_err(|e| AppError::Deserialization(e.to_string(), json.to_string()))
    }

    pub async fn get_menu_items(&self, menu_id: &str) -> Result<Navigation> {
        let url = format!("{MENU_TREE_ENDPOINT}{menu_id}");
        let json = self.fetch_json(&url).await?;

        from_value(json.clone())
            .map_err(|e| AppError::Deserialization(e.to_string(), json.to_string()))
    }

    pub async fn get_resource<T>(&self, resource_url: &str) -> Result<T>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{resource_url}?{PORTFOLIO_QUERY}");
        let json = self.fetch_json(&url).await?;

        from_value(json.clone())
            .map_err(|e| AppError::Deserialization(e.to_string(), json.to_string()))
    }

    pub async fn get_collection<T>(&self, collection_url: &str) -> Result<T>
    where
        T: DeserializeOwned + Debug,
    {
        let json = self.fetch_json(&collection_url).await?;

        from_value(json.clone())
            .map_err(|e| AppError::Deserialization(e.to_string(), json.to_string()))
    }
}
