use async_trait::async_trait;

use crate::cms_content::application::ports::input::GetPortfolioQuery;
use crate::cms_content::application::ports::output::LoadPortfolioPort;
use crate::cms_content::domain::portfolio::Portfolio;

pub struct GetPortfolioService {
    repository: Box<(dyn LoadPortfolioPort)>,
}

#[async_trait(?Send)]
impl GetPortfolioQuery for GetPortfolioService {
    async fn get_portfolio(&self) -> Option<Portfolio> {
        self.repository
            .find_by_slug("/portfolio/santiago-marulanda")
            .await
            .ok()
    }
}

#[derive(Default)]
pub struct GetPortfolioServiceBuilder {
    repository: Option<Box<(dyn LoadPortfolioPort)>>,
}

impl GetPortfolioServiceBuilder {
    pub fn repository(mut self, repository: Box<(dyn LoadPortfolioPort)>) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn build(self) -> GetPortfolioService {
        GetPortfolioService {
            repository: self.repository.expect("repository is required"),
        }
    }
}
