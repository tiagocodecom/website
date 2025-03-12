use async_trait::async_trait;

use crate::adapters::driven::drupal_jsonapi::entities::{ArticleNode, NodeArticleCollection};
use crate::adapters::driven::drupal_jsonapi::mappers::ArticleNodeMapper;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalArticleMapper;
use crate::adapters::driven::drupal_jsonapi::services::{HttpClientService, JsonApiClientService};
use crate::application::domain::article::{Article, Articles};
use crate::application::domain::core::Result;
use crate::application::ports::driven::{ForFetchingArticlesFeatured, ForFetchingArticlesList};

const ARTICLES_COLLECTION_QUERY: &str = "\
    include=thumbnail.media_image,tags\
    &filter[status]=1&sort=created,title\
    &page[limit]=2\
    &sort[sort-created][path]=created&sort[sort-created][direction]=desc\
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
    async fn find_latest_articles(&self) -> Result<Articles> {
        let resource_url = &format!(
            "/jsonapi/node/article?{ARTICLES_COLLECTION_QUERY}&filter[promoted]=1&page[limit]=2"
        );

        let external_articles = self
            .api_client
            .get_collection::<NodeArticleCollection>(resource_url)
            .await?;

        let articles = self
            .api_adapter
            .adapt_multiple(external_articles.data().clone())?;

        Ok(articles)
    }
}

#[async_trait(?Send)]
impl ForFetchingArticlesList for ArticleRepository {
    async fn find_all_articles(&self) -> Result<Vec<Article>> {
        let resource_url =
            &format!("/jsonapi/node/article?{ARTICLES_COLLECTION_QUERY}&page[limit]=10");

        let external_articles = self
            .api_client
            .get_collection::<NodeArticleCollection>(resource_url)
            .await?;

        let articles = self
            .api_adapter
            .adapt_multiple(external_articles.data().clone())?;

        Ok(articles.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::driven::drupal_jsonapi::services::Config;
    use mockito::Matcher::Regex;
    use mockito::{Mock, Server};

    #[actix_rt::test]
    async fn succeeds_to_fetch_latest_articles() {
        let mut server = Server::new_async().await;

        let collection_mock = collection_mock(&mut server, "/jsonapi/node/article", 200).await;
        let http_client = http_client_mock(&server.url());

        let articles = ArticleRepository::new(http_client)
            .find_latest_articles()
            .await
            .unwrap();

        assert!(collection_mock.matched_async().await);
        assert_eq!(articles.len(), 2);
    }

    #[actix_rt::test]
    async fn succeeds_to_fetch_all_articles() {
        let mut server = Server::new_async().await;

        let collection_mock = collection_mock(&mut server, "/jsonapi/node/article", 200).await;
        let http_client = http_client_mock(&server.url());

        let articles = ArticleRepository::new(http_client)
            .find_all_articles()
            .await
            .unwrap();

        assert!(collection_mock.matched_async().await);
        assert_eq!(articles.len(), 2);
    }

    fn http_client_mock(url: &str) -> HttpClientService {
        HttpClientService::new(
            Config::default()
                .base_url(url.to_string())
                .basic_auth(("user", "password"))
                .build(),
        )
    }

    async fn collection_mock(server: &mut Server, path: &str, status: usize) -> Mock {
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
