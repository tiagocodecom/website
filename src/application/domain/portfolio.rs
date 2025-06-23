use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::common::MetaTags;
use crate::application::domain::portfolio_section::PortfolioSection;
use crate::application::value_objects::{Date, Identifier, ModerationStatus, RequiredText};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Portfolio {
    id: Identifier,
    title: RequiredText,
    created_at: Date,
    status: ModerationStatus,
    metatags: MetaTags,
    sections: Vec<PortfolioSection>,
}

impl Portfolio {
    pub fn sections_mut(&mut self) -> &mut Vec<PortfolioSection> {
        &mut self.sections
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::application::domain::portfolio_section::tests::{
        about_me_fixture, blogs_fixture, projects_fixture, resume_fixture,
    };

    #[test]
    fn creation_succeeds_when_valid_portfolio() {
        portfolio_fixture(); // the fixture calls the builder methods, so we just ensure that it doesn't panic
    }

    #[test]
    fn serialization_succeeds_when_valid_portfolio() {
        let p = portfolio_fixture();
        let serialized = serde_json::json!(&p).to_string();

        assert!(serialized.contains(p.id().to_string().as_str()));
        assert!(serialized.contains(p.title().as_str()));
        assert!(serialized.contains(p.status().to_string().as_str()));
    }

    #[test]
    fn deserialization_succeeds_when_valid_portfolio() {
        let p = portfolio_fixture();
        let serialized = serde_json::json!(&p).to_string();
        let deserialized: Portfolio = serde_json::from_str(&serialized).unwrap();

        assert_eq!(p.id(), deserialized.id());
        assert_eq!(p.title(), deserialized.title());
        assert_eq!(p.status(), deserialized.status());
        assert_eq!(p.sections().len(), deserialized.sections().len());
    }

    pub fn portfolio_fixture() -> Portfolio {
        PortfolioBuilder::default()
            .id(Identifier::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap())
            .title(RequiredText::try_from("Portfolio John Doe").unwrap())
            .status(ModerationStatus::Published)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .sections(sections_fixture())
            .build()
            .unwrap()
    }

    pub fn unpublished_portfolio_fixture() -> Portfolio {
        PortfolioBuilder::default()
            .id(Identifier::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap())
            .title(RequiredText::try_from("Portfolio John Doe").unwrap())
            .status(ModerationStatus::Unpublished)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .sections(sections_fixture())
            .build()
            .unwrap()
    }

    fn sections_fixture() -> Vec<PortfolioSection> {
        vec![
            PortfolioSection::Resume(resume_fixture()),
            PortfolioSection::AboutMe(about_me_fixture()),
            PortfolioSection::Projects(projects_fixture()),
            PortfolioSection::Blogs(blogs_fixture()),
        ]
    }
}
