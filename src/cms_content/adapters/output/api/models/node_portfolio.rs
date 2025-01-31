//! Node Portfolio Models
//!
//! This module provides models for handling node portfolio resources, which represent
//! portfolio pages in the API. It includes metadata about the portfolio page and
//! the sections (paragraphs) that compose its content.
use derive_getters::Getters;
use leptos::server_fn::serde;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;

use crate::cms_content::adapters::output::api::models::{FieldContent, FieldPath};

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct NodePortfolio {
    id: String,
    title: String,
    path: FieldPath,
    content: Vec<FieldContent>,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(rename = "langcode")]
    lang_code: String,
    #[serde(rename = "created")]
    created_at: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_node_portfolio() {
        let json_str = mock_node_portfolio();
        let node_portfolio = serde_json::from_str::<NodePortfolio>(json_str).unwrap();

        assert_eq!(*node_portfolio.id(), "1");
        assert_eq!(node_portfolio.title(), "Portfolio");
        assert_eq!(node_portfolio.path().alias(), "test");
        assert_eq!(node_portfolio.path().lang_code(), "en");
        assert_eq!(*node_portfolio.path().pid(), 1);
        assert_eq!(node_portfolio.content().len(), 0);
        assert_eq!(node_portfolio.entity_type(), "node--portfolio");
        assert_eq!(node_portfolio.lang_code(), "en");
        assert_eq!(node_portfolio.created_at(), "2025-01-01");
        assert_eq!(*node_portfolio.status(), true);
    }

    fn mock_node_portfolio() -> &'static str {
        r#"
        {
            "id": "1",
            "title": "Portfolio",
            "path": {
                "pid": "1",
                "langcode": "en",
                "alias": "test"
            },
            "content": [],
            "type": "node--portfolio",
            "langcode": "en",
            "created": "2025-01-01",
            "status": true
        }
        "#
    }
}
