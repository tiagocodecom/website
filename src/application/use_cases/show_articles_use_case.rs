use async_trait::async_trait;

use crate::application::domain::article::{Article, Category};
use crate::application::domain::core::Result;
use crate::application::ports::driven::{ForFetchingArticlesList, ForFetchingCategoriesList};
use crate::application::ports::driver::ForListingArticles;

/// Service for retrieving articles and categories data
///
/// This use case implements the ForDisplayingArticles interface and uses repositories
/// that implement ForFetchingArticlesList and ForFetchingCategoriesList to fetch
/// the necessary data for displaying articles with their respective categories.
pub struct ShowArticlesUseCase {
    articles_repository: Box<dyn ForFetchingArticlesList>,
    categories_repository: Box<dyn ForFetchingCategoriesList>,
}

impl ShowArticlesUseCase {
    pub fn new(
        articles_repository: Box<dyn ForFetchingArticlesList>,
        categories_repository: Box<dyn ForFetchingCategoriesList>,
    ) -> Self {
        Self {
            articles_repository,
            categories_repository,
        }
    }
}

#[async_trait(?Send)]
impl ForListingArticles for ShowArticlesUseCase {
    async fn execute(&self) -> Result<(Vec<Category>, Vec<Article>)> {
        let articles = self.articles_repository.find_all_articles().await?;
        let categories = self.categories_repository.find_all_categories().await?;

        Ok((categories, articles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::{Article, ArticleBuilder};
    use crate::application::domain::article::{Category, CategoryBuilder};
    use crate::application::domain::common::{Image, ImageBuilder};
    use crate::application::value_objects::ModerationStatus;

    #[actix_rt::test]
    async fn executor_succeeds_to_fetch_articles_and_categories() {
        let articles = vec![article_fixture(), article_fixture()];
        let categories = vec![category_fixture(), category_fixture()];
        let articles_repository = ArticlesRepositoryMock::with_fixture(articles);
        let categories_repository = CategoriesRepositoryMock::with_fixture(categories);

        let use_case = ShowArticlesUseCase::new(
            Box::new(articles_repository),
            Box::new(categories_repository),
        );

        let (fetched_categories, fetched_articles) = use_case.execute().await.unwrap();

        assert_eq!(fetched_categories.len(), 2);
        assert_eq!(fetched_articles.len(), 2);
    }

    struct ArticlesRepositoryMock {
        fixture: Vec<Article>,
    }

    struct CategoriesRepositoryMock {
        fixture: Vec<Category>,
    }

    impl ArticlesRepositoryMock {
        pub fn with_fixture(fixture: Vec<Article>) -> Self {
            Self { fixture }
        }
    }

    impl CategoriesRepositoryMock {
        pub fn with_fixture(fixture: Vec<Category>) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingArticlesList for ArticlesRepositoryMock {
        async fn find_all_articles(&self) -> Result<Vec<Article>> {
            Ok(self.fixture.clone())
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingCategoriesList for CategoriesRepositoryMock {
        async fn find_all_categories(&self) -> Result<Vec<Category>> {
            Ok(self.fixture.clone())
        }
    }

    fn article_fixture() -> Article {
        ArticleBuilder::default()
            .id("4f6c9689-41e7-4a1d-bc43-5d3f98771300".try_into().unwrap())
            .slug("/what-is-lorem-ipsum".try_into().unwrap())
            .title("What is Lorem Ipsum?".try_into().unwrap())
            .summary("Lorem Ipsum is simply dummy text".try_into().unwrap())
            .status(ModerationStatus::Published)
            .created_at("2024-12-15T14:03:56+00:00".try_into().unwrap())
            .category(category_fixture())
            .thumbnail(thumbnail_fixture())
            .sections(vec![])
            .build()
            .unwrap()
    }

    fn category_fixture() -> Category {
        CategoryBuilder::default()
            .id("4f6c9689-41e7-4a1d-bc43-5d3f98771300".try_into().unwrap())
            .title("Writing".try_into().unwrap())
            .status(ModerationStatus::Published)
            .emoji("✏️️".try_into().unwrap())
            .build()
            .unwrap()
    }

    fn thumbnail_fixture() -> Image {
        ImageBuilder::default()
            .id("4f6c9689-41e7-4a1d-bc43-5d3f98771300".try_into().unwrap())
            .url("https://example.com/image.jpg".try_into().unwrap())
            .alt("Lorem ipsum image".try_into().unwrap())
            .title("Lorem ipsum image".try_into().unwrap())
            .width(100)
            .height(100)
            .build()
            .unwrap()
    }
}
