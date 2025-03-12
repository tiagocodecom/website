use async_trait::async_trait;

use crate::adapters::driven::drupal_jsonapi::entities::Navigation;
use crate::adapters::driven::drupal_jsonapi::mappers::{ExternalMenuTreeMapper, NavigationAdapter};
use crate::adapters::driven::drupal_jsonapi::services::{HttpClientService, JsonApiClientService};
use crate::application::domain::core::Result;
use crate::application::domain::layout::MenuTree;
use crate::application::ports::driven::ForFetchingMenuData;

/// Repository for fetching and transforming menu data from an external CMS API.
///
/// This struct implements the `ForFetchingMenuData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve menu items and transform them into domain entities.
///
/// # Dependencies
/// - `api_client`: A connector to the CMS API for fetching menu data
/// - `api_mapper`: A mapper to convert external menu items into domain `MenuTree` objects
pub struct LayoutRepository {
    api_client: Box<JsonApiClientService>,
    api_mapper: Box<(dyn ExternalMenuTreeMapper<Input = Navigation>)>,
}

impl LayoutRepository {
    pub fn new(http_client: HttpClientService) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client)),
            api_mapper: Box::new(NavigationAdapter::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingMenuData for LayoutRepository {
    async fn find_menu_by_id(&self, id: &str) -> Result<MenuTree> {
        let external_menu_tree = self.api_client.get_menu_items(id).await?;

        Ok(self.api_mapper.adapt(external_menu_tree)?)
    }
}
