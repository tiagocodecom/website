use serde::{Deserialize, Serialize};

/// Different sections that compose the article body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticleSection {
    Unknown,
}
