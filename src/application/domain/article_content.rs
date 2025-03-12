use crate::application::domain::common::Image;
use crate::application::value_objects::RequiredText;
use serde::{Deserialize, Serialize};

/// Different sections that compose the article body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticleContent {
    Image(Image),
    Text(RequiredText),
    Unknown,
}
