use async_trait::async_trait;

use crate::cms_content::domain::error::CmsResult;
use crate::cms_content::domain::portfolio::Portfolio;

/// Port for loading portfolio data in the CMS context.
///
/// This output port defines the contract for retrieving portfolio entities
/// based on their slug. Implementations should handle data access, whether
/// from a database, API, or other storage mechanisms.
///
/// # Errors
/// Returns a `CmsResult` wrapping a `Portfolio` on success, or an error
/// indicating the failure reason.
#[async_trait(?Send)]
pub trait LoadPortfolioPort {
    /// Finds a portfolio by its slug.
    async fn find_by_slug(&self, slug: &str) -> CmsResult<Portfolio>;
}

/// Defines a contract for transforming external data representations into a `Portfolio` domain entity.
///
/// This trait is responsible for converting an external data format (such as an API response,
/// database model, or DTO) into the internal `Portfolio` structure used within the system.
/// It ensures a clear separation between external data sources and the core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type that will be transformed into a `Portfolio`.
///
/// # Errors
/// Returns a `CmsResult<Portfolio>`:
/// - On success, provides a mapped `Portfolio` entity.
/// - On failure, returns an error indicating the reason for the transformation failure.
pub trait LoadPortfolioStateMapper {
    type Input;

    /// Transforms an external data representation into a `Portfolio` domain entity.
    ///
    /// # Parameters
    /// - `input`: The external data structure to be converted into a `Portfolio`.
    ///
    /// # Returns
    /// - `CmsResult<Portfolio>`: A successfully mapped `Portfolio` or an error if transformation fails.
    fn transform(&self, input: Self::Input) -> CmsResult<Portfolio>;
}
