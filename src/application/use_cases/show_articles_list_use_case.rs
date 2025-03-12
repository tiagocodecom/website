use async_trait::async_trait;

use crate::application::domain::article::{Article, Category};
use crate::application::domain::core::Result;
use crate::application::ports::driven::{ForFetchingArticlesList, ForFetchingCategoriesList};
use crate::application::ports::driver::ForDisplayingArticlesList;
use crate::application::value_objects::ModerationStatus;

/// Service for retrieving articles and categories data
///
/// This use case implements the ForDisplayingArticles interface and uses repositories
/// that implement ForFetchingArticlesList and ForFetchingCategoriesList to fetch
/// the necessary data for displaying articles with their respective categories.
pub struct ShowArticlesListUseCase {
    article_repository: Box<dyn ForFetchingArticlesList>,
    category_repository: Box<dyn ForFetchingCategoriesList>,
}

impl ShowArticlesListUseCase {
    pub fn new(
        article_repository: Box<dyn ForFetchingArticlesList>,
        category_repository: Box<dyn ForFetchingCategoriesList>,
    ) -> Self {
        Self {
            article_repository,
            category_repository,
        }
    }
}

#[async_trait(?Send)]
impl ForDisplayingArticlesList for ShowArticlesListUseCase {
    async fn execute(&self) -> Result<(Vec<Category>, Vec<Article>)> {
        let articles = self
            .article_repository
            .get_list()
            .await?
            .into_iter()
            .filter(|a| a.status().eq(&ModerationStatus::Published))
            .collect();

        let categories = self.category_repository.find_all_categories().await?;

        Ok((categories, articles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::tests::{
        article_fixture, category_fixture, unpublished_article_fixture,
    };
    use crate::application::domain::article::Article;
    use crate::application::domain::article::Category;

    struct ArticleRepositoryMock {
        fixture: Vec<Article>,
    }

    struct CategoryRepositoryMock {
        fixture: Vec<Category>,
    }

    impl ArticleRepositoryMock {
        pub fn with_fixture(fixture: Vec<Article>) -> Self {
            Self { fixture }
        }
    }

    impl CategoryRepositoryMock {
        pub fn with_fixture(fixture: Vec<Category>) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingArticlesList for ArticleRepositoryMock {
        async fn get_list(&self) -> Result<Vec<Article>> {
            Ok(self.fixture.clone())
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingCategoriesList for CategoryRepositoryMock {
        async fn find_all_categories(&self) -> Result<Vec<Category>> {
            Ok(self.fixture.clone())
        }
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_valid_articles_and_categories_are_retrieved() {
        let articles_fixture = vec![article_fixture(), article_fixture()];
        let categories_fixture = vec![category_fixture()];
        let article_repo_mock = ArticleRepositoryMock::with_fixture(articles_fixture.clone());
        let category_repo_mock = CategoryRepositoryMock::with_fixture(categories_fixture.clone());

        let use_case =
            ShowArticlesListUseCase::new(Box::new(article_repo_mock), Box::new(category_repo_mock));
        let (fetched_categories, fetched_articles) = use_case.execute().await.unwrap();

        assert_eq!(fetched_articles.len(), articles_fixture.len());
        assert_eq!(fetched_categories.len(), categories_fixture.len());
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_excluding_unpublished_articles() {
        let articles_fixture = vec![article_fixture(), unpublished_article_fixture()];
        let categories_fixture = vec![category_fixture()];
        let article_repo_mock = ArticleRepositoryMock::with_fixture(articles_fixture.clone());
        let category_repo_mock = CategoryRepositoryMock::with_fixture(categories_fixture.clone());

        let use_case =
            ShowArticlesListUseCase::new(Box::new(article_repo_mock), Box::new(category_repo_mock));
        let (fetched_categories, fetched_articles) = use_case.execute().await.unwrap();

        assert_eq!(fetched_articles.len(), 1);
        assert_eq!(fetched_categories.len(), categories_fixture.len());
    }
}
