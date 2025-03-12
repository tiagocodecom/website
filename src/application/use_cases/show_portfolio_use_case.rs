use crate::application::domain::core::{AppError, Result};
use crate::application::domain::portfolio::Portfolio;
use crate::application::domain::portfolio_section::{BlogsBuilder, PortfolioSection};
use crate::application::ports::driven::{ForFetchingArticlesData, ForFetchingPortfolioData};
use crate::application::ports::driver::ForDisplayingPortfolio;
use crate::application::value_objects::ModerationStatus;
use async_trait::async_trait;

/// Service for retrieving portfolio data
///
/// This service implements the GetPortfolioQuery interface and uses a repository
/// that implements LoadPortfolioPort to fetch portfolio data.
///
pub struct ShowPortfolioUseCase {
    portfolio_repository: Box<dyn ForFetchingPortfolioData>,
    articles_repository: Box<dyn ForFetchingArticlesData>,
}

impl ShowPortfolioUseCase {
    pub fn new(
        portfolio_repository: Box<(dyn ForFetchingPortfolioData)>,
        articles_repository: Box<(dyn ForFetchingArticlesData)>,
    ) -> Self {
        Self {
            portfolio_repository,
            articles_repository,
        }
    }
}

#[async_trait(?Send)]
impl ForDisplayingPortfolio for ShowPortfolioUseCase {
    async fn execute(&self) -> Result<Portfolio> {
        let mut portfolio = self
            .portfolio_repository
            .find_portfolio_by_slug("/portfolio/santiago-marulanda")
            .await?;

        if portfolio.status().eq(&ModerationStatus::Unpublished) {
            return Err(AppError::Unauthorized("Unpublished".to_string()));
        }

        let articles = self.articles_repository.find_latest_articles().await?;

        for section in portfolio.sections_mut() {
            if let PortfolioSection::Blogs(b) = section {
                let updated_blog = BlogsBuilder::default()
                    .id(b.id().clone())
                    .title(b.title().clone())
                    .subtitle(b.subtitle().clone())
                    .text(b.text().clone())
                    .articles(articles.to_vec())
                    .build()
                    .unwrap();

                *section = PortfolioSection::Blogs(updated_blog);
            }
        }

        Ok(portfolio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::{Article, Articles};
    use crate::application::domain::portfolio::PortfolioBuilder;

    struct ArticleRepositoryMock {
        fixture: Vec<Article>,
    }

    impl ArticleRepositoryMock {
        pub fn with_fixture(fixture: Vec<Article>) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingArticlesData for ArticleRepositoryMock {
        async fn find_latest_articles(&self) -> Result<Articles> {
            Ok(self.fixture.clone())
        }
    }

    struct PortfolioRepositoryMock {
        fixture: Portfolio,
    }

    impl PortfolioRepositoryMock {
        pub fn with_fixture(fixture: Portfolio) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingPortfolioData for PortfolioRepositoryMock {
        async fn find_portfolio_by_slug(&self, slug: &str) -> Result<Portfolio> {
            Ok(self.fixture.clone())
        }
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_portfolio_is_published() {
        let portfolio_fixture = PortfolioBuilder::default()
            .id("4f6c9689-41e7-4a1d-bc43-5d3f98771300".try_into().unwrap())
            .title("John Doe".try_into().unwrap())
            .status(ModerationStatus::Published)
            .sections(vec![])
            .created_at("2024-12-15T14:03:56+00:00".try_into().unwrap())
            .build()
            .unwrap();

        let portfolio_repository_mock = PortfolioRepositoryMock::with_fixture(portfolio_fixture);
        let article_repository_mock = ArticleRepositoryMock::with_fixture(vec![]);

        let use_case = ShowPortfolioUseCase::new(
            Box::new(portfolio_repository_mock),
            Box::new(article_repository_mock),
        );
        let result = use_case.execute().await.unwrap();

        assert_eq!(result.title().to_string(), "John Doe");
        assert_eq!(result.status(), &ModerationStatus::Published);
    }

    #[actix_rt::test]
    async fn executor_returns_error_when_portfolio_is_unpublished() {
        let portfolio_fixture = PortfolioBuilder::default()
            .id("4f6c9689-41e7-4a1d-bc43-5d3f98771300".try_into().unwrap())
            .title("John Doe".try_into().unwrap())
            .status(ModerationStatus::Unpublished)
            .created_at("2024-12-15T14:03:56+00:00".try_into().unwrap())
            .sections(vec![])
            .build()
            .unwrap();

        let portfolio_repository_mock = PortfolioRepositoryMock::with_fixture(portfolio_fixture);
        let article_repository_mock = ArticleRepositoryMock::with_fixture(vec![]);

        let use_case = ShowPortfolioUseCase::new(
            Box::new(portfolio_repository_mock),
            Box::new(article_repository_mock),
        );
        let result = use_case.execute().await;

        assert!(matches!(result, Err(AppError::Unauthorized(_))));
    }
}
