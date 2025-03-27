use async_trait::async_trait;

use crate::application::domain::article::{Article, Category};
use crate::application::domain::core::Result;

/// Use case for listing the articles.
///
/// This input port defines the application logic for fetching all articles.
/// It acts as a boundary between the web UI (controller) and the application core,
/// allowing the web UI to obtain articles data for rendering.
#[async_trait(?Send)]
pub trait ForDisplayingArticlesList {
    /// Retrieves a list of articles, and categories.
    ///
    /// # Returns
    /// * `AppError<(Vec<Category>, Vec<Article>)>` - A result wrapping the `Categories` and
    ///   `Articles` if found, or an error if the articles could not be retrieved.
    async fn execute(&self, category_name: Option<String>)
        -> Result<(Vec<Category>, Vec<Article>)>;
}
