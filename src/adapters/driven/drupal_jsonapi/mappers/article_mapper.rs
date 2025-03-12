use crate::adapters::driven::drupal_jsonapi::entities::{ArticleNode, ImageField, TagsVocabulary};
use crate::application::domain::article::{Article, ArticleBuilder, Articles};
use crate::application::domain::article::{Category, CategoryBuilder};
use crate::application::domain::common::{Image, ImageBuilder};
use crate::application::domain::core::{AppError, Result};

/// Trait for converting external data into an `Article` domain entity.
/// Ensures separation between external data sources and core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type to be transformed into an `Article`.
pub trait ExternalArticleMapper {
    type Input;

    /// Converts external data into an `Article`.
    ///
    /// # Arguments
    /// * `input` - The external data to be transformed into an `Article`.
    ///
    /// # Returns
    /// * `Result<Article>` - The result of the transformation.
    fn adapt(&self, input: Self::Input) -> Result<Article>;

    fn adapt_multiple(&self, input: Vec<Self::Input>) -> Result<Articles> {
        input
            .into_iter()
            .map(|article| self.adapt(article))
            .collect()
    }
}

#[derive(Default)]
pub struct ArticleNodeMapper;

impl ExternalArticleMapper for ArticleNodeMapper {
    type Input = ArticleNode;

    fn adapt(&self, input: Self::Input) -> Result<Article> {
        article_node_mapper(input)
    }
}

fn article_node_mapper(node: ArticleNode) -> Result<Article> {
    ArticleBuilder::default()
        .id(node.id().to_string().try_into()?)
        .slug(node.path().alias().to_string().try_into()?)
        .status(node.status().clone().into())
        .title(node.title().to_string().try_into()?)
        .summary(node.body().to_string().try_into()?)
        .created_at(node.created_at().to_string().try_into()?)
        .category(tag_vocabulary_mapper(node.tags().clone()))
        .thumbnail(thumbnail_field_mapper(node.thumbnail()))
        .sections(vec![])
        .build()
        .map_err(|e| AppError::Unexpected(e.to_string()))
}

fn tag_vocabulary_mapper(tag: TagsVocabulary) -> Category {
    CategoryBuilder::default()
        .id(tag.id().to_string().try_into().unwrap())
        .title(tag.name().to_string().try_into().unwrap())
        .status(tag.status().clone().into())
        .emoji(tag.emoji().to_string().try_into().unwrap())
        .build()
        .unwrap()
}

fn thumbnail_field_mapper(p: &ImageField) -> Image {
    let url = p
        .media_image()
        .image_style_uri()
        .thumbnail_260x210()
        .to_string()
        .try_into()
        .unwrap();

    ImageBuilder::default()
        .id(p.id().to_string().try_into().unwrap())
        .title(p.media_image().meta().alt().to_string().try_into().unwrap())
        .alt(p.media_image().meta().alt().to_string().try_into().unwrap())
        .height(p.media_image().meta().height().clone())
        .width(p.media_image().meta().width().clone())
        .url(url)
        .build()
        .unwrap()
}
