use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::portfolio_section::PortfolioSection;
use crate::application::value_objects::{Date, Identifier, ModerationStatus, RequiredText};

/// Represents a complete portfolio with various sections
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Portfolio {
    id: Identifier,
    title: RequiredText,
    created_at: Date,
    status: ModerationStatus,
    sections: Vec<PortfolioSection>,
}

impl Portfolio {
    pub fn sections_mut(&mut self) -> &mut Vec<PortfolioSection> {
        &mut self.sections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_succeeds_for_portfolio() {
        let p = portfolio_fixture();
        let serialized = serde_json::json!(&p).to_string();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("Published"));
        assert!(serialized.contains("Portfolio Santiago Marulanda"));
    }

    #[test]
    fn deserialization_succeeds_for_portfolio() {
        let json_str = json_portfolio_fixture();
        let p = serde_json::from_str::<Portfolio>(json_str).unwrap();

        assert_eq!(p.id().to_string(), "413b8ba1-2bc4-4fda-8455-0c0dea528ca0");
        assert_eq!(p.title().to_string(), "Portfolio Santiago Marulanda");
        assert_eq!(p.status().clone(), ModerationStatus::Published);
    }

    fn portfolio_fixture() -> Portfolio {
        PortfolioBuilder::default()
            .id(Identifier::try_from("413b8ba1-2bc4-4fda-8455-0c0dea528ca0").unwrap())
            .title(RequiredText::try_from("Portfolio Santiago Marulanda").unwrap())
            .status(ModerationStatus::Published)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .sections(vec![])
            .build()
            .unwrap()
    }

    fn json_portfolio_fixture() -> &'static str {
        r#"{
            "created_at": "2024-12-15T14:03:56Z",
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "sections": [],
            "status": "Published",
            "title": "Portfolio Santiago Marulanda"
        }"#
    }
}
