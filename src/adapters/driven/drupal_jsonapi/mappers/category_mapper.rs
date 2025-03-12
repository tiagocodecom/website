use crate::adapters::driven::drupal_jsonapi::entities::TagsVocabulary;
use crate::application::domain::article::Category;
use crate::application::domain::core::Result;

/// Trait for converting external data into a `Category` domain entity.
/// Ensures separation between external data sources and core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type to be transformed into a `Category`.
pub trait ExternalCategoryMapper {
    type Input;

    /// Converts external data into a `Category`.
    ///
    /// # Arguments
    /// * `input` - The external data to be transformed into a `Category`.
    ///
    /// # Returns
    /// * `Result<Category>` - The result of the transformation.
    fn adapt(&self, input: Self::Input) -> Result<Category>;

    fn adapt_multiple(&self, input: Vec<Self::Input>) -> Result<Vec<Category>> {
        input
            .into_iter()
            .map(|category| self.adapt(category))
            .collect()
    }
}

#[derive(Default)]
pub struct ExternalTagsVocabularyMapper;

impl ExternalCategoryMapper for ExternalTagsVocabularyMapper {
    type Input = TagsVocabulary;

    fn adapt(&self, input: Self::Input) -> Result<Category> {
        todo!()
    }
}
