use async_trait::async_trait;
use std::any::type_name;

use crate::adapters::driven::drupal_jsonapi::entities::{
    ArticleNode, NodeArticleCollection, NodeArticleResource,
};
use crate::adapters::driven::drupal_jsonapi::mappers::ArticleNodeMapper;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalArticleMapper;
use crate::adapters::driven::drupal_jsonapi::services::{HttpClientService, JsonApiClientService};
use crate::application::domain::article::{Article, Articles};
use crate::application::domain::core::{AppError, Result};
use crate::application::ports::driven::{
    ForFetchingArticleData, ForFetchingArticlesFeatured, ForFetchingArticlesList,
};

const COLLECTION_QUERY: &str = "\
    include=tags,content.media.media_image,thumbnail.media_image\
    &filter[status]=1&sort=created,title\
    &page[limit]=2\
    &sort[sort-created][path]=created&sort[sort-created][direction]=desc\
    &jsonapi_include=1";

const RESOURCE_QUERY: &str = "\
    include=tags,content.media.media_image,thumbnail.media_image\
    &jsonapi_include=1";

/// Repository for fetching and transforming article data from an external CMS API.
///
/// This struct implements the `ForFetchingArticleData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve portfolio items and transform them into domain entities.
///
/// # Dependencies
/// - `api_client`: A connector to the CMS API for fetching portfolio data
/// - `api_mapper`: A mapper to convert external portfolio items into domain `Article` objects
pub struct ArticleRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<(dyn ExternalArticleMapper<Input = ArticleNode>)>,
}

impl ArticleRepository {
    pub fn new(http_client: HttpClientService) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client)),
            api_adapter: Box::new(ArticleNodeMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingArticlesFeatured for ArticleRepository {
    async fn get_featured(&self) -> Result<Articles> {
        let external_endpoint =
            &format!("/jsonapi/node/article?{COLLECTION_QUERY}&filter[promoted]=1&page[limit]=2");

        let external_articles = self
            .api_client
            .get_external_data::<NodeArticleCollection>(external_endpoint)
            .await
            .map_err(|e| AppError::External(type_name::<Self>(), e.to_string()))?;

        let articles = self
            .api_adapter
            .adapt_multiple(external_articles.data().clone())?;

        Ok(articles)
    }
}

#[async_trait(?Send)]
impl ForFetchingArticlesList for ArticleRepository {
    async fn get_list(&self) -> Result<Vec<Article>> {
        let adapter = type_name::<Self>();
        let endpoint = &format!("/jsonapi/node/article?{COLLECTION_QUERY}&page[limit]=10");

        let articles = self
            .api_client
            .get_external_data::<NodeArticleCollection>(endpoint)
            .await
            .map_err(|e| AppError::External(adapter, e.to_string()))?;

        Ok(self
            .api_adapter
            .adapt_multiple(articles.data().clone())?
            .into_iter()
            .collect())
    }
}

#[async_trait(?Send)]
impl ForFetchingArticleData for ArticleRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Article> {
        let adapter = type_name::<Self>();
        let endpoint = self
            .api_client
            .resolve_external_endpoint(slug)
            .await
            .map_err(|e| AppError::External(adapter, e.to_string()))?;
        let endpoint = &format!("{endpoint}?{RESOURCE_QUERY}");

        let article = self
            .api_client
            .get_external_data::<NodeArticleResource>(endpoint)
            .await
            .map_err(|e| AppError::External(adapter, e.to_string()))?;

        Ok(self.api_adapter.adapt(article.data().clone())?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use mockito::Matcher::Regex;
    use mockito::{Mock, Server};

    use crate::adapters::driven::drupal_jsonapi::services::http_client_service::tests::http_client_mock;

    #[actix_rt::test]
    async fn fetching_details_succeeds_when_api_response_is_valid() {
        let mut server = Server::new_async().await;
        let router_mock = router_mock(&mut server, "/articles/organizing-data-memory", 200).await;
        let request_mock = collection_request_mock(&mut server, "/jsonapi/node/article", 200).await;
        let http_client_mock = http_client_mock(&server.url());

        let article = ArticleRepository::new(http_client_mock)
            .find_by_slug()
            .await
            .unwrap();

        assert!(request_mock.matched_async().await);
        assert!(router_mock.matched_async().await);
        assert_ne!(article.slug().to_string(), "");
        assert_ne!(article.title().to_string(), "");
        assert_ne!(article.summary().to_string(), "");
        assert_ne!(article.status().to_string(), "");
        assert_ne!(article.thumbnail().url().to_string(), "");
        assert_ne!(article.category().title().to_string(), "");
    }

    #[actix_rt::test]
    async fn fetching_featured_succeeds_when_api_response_is_valid() {
        let mut server = Server::new_async().await;
        let request_mock = collection_request_mock(&mut server, "/jsonapi/node/article", 200).await;
        let http_client_mock = http_client_mock(&server.url());

        let articles = ArticleRepository::new(http_client_mock)
            .get_featured()
            .await
            .unwrap();

        let first_article = articles.first().unwrap();

        assert!(request_mock.matched_async().await);
        assert!(articles.len() > 0);
        assert_ne!(first_article.slug().to_string(), "");
        assert_ne!(first_article.title().to_string(), "");
        assert_ne!(first_article.summary().to_string(), "");
        assert_ne!(first_article.status().to_string(), "");
        assert_ne!(first_article.thumbnail().url().to_string(), "");
        assert_ne!(first_article.category().title().to_string(), "");
    }

    #[actix_rt::test]
    async fn fetching_list_succeeds_when_api_response_is_valid() {
        let mut server = Server::new_async().await;
        let request_mock = collection_request_mock(&mut server, "/jsonapi/node/article", 200).await;
        let http_client_mock = http_client_mock(&server.url());

        let articles = ArticleRepository::new(http_client_mock)
            .get_list()
            .await
            .unwrap();
        let first_article = articles.first().unwrap();

        assert!(request_mock.matched_async().await);
        assert!(articles.len() > 0);
        assert_ne!(first_article.slug().to_string(), "");
        assert_ne!(first_article.title().to_string(), "");
        assert_ne!(first_article.summary().to_string(), "");
        assert_ne!(first_article.status().to_string(), "");
        assert_ne!(first_article.thumbnail().url().to_string(), "");
        assert_ne!(first_article.category().title().to_string(), "");
    }

    async fn router_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        let path = format!("/router/translate-path?path={path}");
        let body_file = &format!("tests/fixtures/http_article_decoupled_router_{status}.json");

        server
            .mock("GET", path.as_str())
            .with_status(status)
            .with_header("content-type", "application/json")
            .with_body_from_file(body_file)
            .create_async()
            .await
    }

    async fn collection_request_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        let body_file_path = &format!("tests/fixtures/http_articles_collection_{status}.json");

        server
            .mock("GET", Regex(path.to_string()))
            .with_status(status)
            .with_header("content-type", "application/vnd.api+json")
            .with_body_from_file(body_file_path)
            .create_async()
            .await
    }
}
