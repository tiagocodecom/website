use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;

use crate::adapters::driven::drupal_jsonapi::entities::{ContentField, ImageField, PathField};

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ArticleNode {
    id: String,
    title: String,
    path: PathField,
    thumbnail: ImageField,
    body: String,
    content: Vec<ContentField>,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(rename = "langcode")]
    lang_code: String,
    #[serde(rename = "created")]
    created_at: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PortfolioNode {
    id: String,
    title: String,
    path: PathField,
    content: Vec<ContentField>,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(rename = "langcode")]
    lang_code: String,
    #[serde(rename = "created")]
    created_at: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
}
