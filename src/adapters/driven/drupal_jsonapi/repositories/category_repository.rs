use async_trait::async_trait;

use crate::adapters::driven::drupal_jsonapi::entities::TagsVocabulary;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalCategoryMapper;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalTagsVocabularyMapper;
use crate::adapters::driven::drupal_jsonapi::services::{HttpClientService, JsonApiClientService};
use crate::application::domain::article::Category;
use crate::application::ports::driven::ForFetchingCategoriesList;

/// Repository for fetching and transforming category data from an external API.
///
/// This struct implements a data access layer by integrating with a CMS API client
/// to retrieve category information and transform it into domain entities using
/// the adapter pattern.
pub struct CategoryRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<(dyn ExternalCategoryMapper<Input = TagsVocabulary>)>,
}

impl CategoryRepository {
    pub fn new(http_client: HttpClientService) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client)),
            api_adapter: Box::new(ExternalTagsVocabularyMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingCategoriesList for CategoryRepository {
    async fn find_all_categories(&self) -> crate::application::domain::core::Result<Vec<Category>> {
        Ok(vec![])
    }
}
