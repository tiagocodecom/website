use async_trait::async_trait;

use crate::application::domain::article::Article;
use crate::application::domain::core::Result;

/// Use case for retrieving article data.
///
/// This input port defines the application logic for fetching an article.
/// It acts as a boundary between the web UI (controller) and the application core,
/// allowing the web UI to obtain article data for rendering.
#[async_trait(?Send)]
pub trait ForDisplayingArticle {
    /// Retrieves an article by its path.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path of the article to be retrieved.
    ///
    /// # Returns
    /// * `CmsResult<Article>` - A result wrapping the `Article` if found, or an error
    ///   if the article could not be retrieved.
    async fn execute(&self, path: &str) -> Result<Article>;
}
