use async_trait::async_trait;

use crate::cms_content::domain::portfolio::Portfolio;

/// Use case for retrieving portfolio data.
///
/// This input port defines the application logic for fetching a portfolio.
/// It acts as a boundary between the application core and external adapters
/// (e.g., repositories, APIs).
///
/// # Errors
/// Returns a `CmsResult` containing a `Portfolio` on success, or an error
/// if the operation fails.
#[async_trait(?Send)]
pub trait GetPortfolioQuery {
    /// Retrieves a portfolio.
    async fn get_portfolio(&self) -> Option<Portfolio>;
}
