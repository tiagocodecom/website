use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::article_section::ArticleSection;
use crate::application::domain::common::Image;
use crate::application::value_objects::{Date, Identifier, ModerationStatus, RequiredText, Url};

pub type Articles = Vec<Article>;

/// Represents a complete article with various sections
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Article {
    id: Identifier,
    slug: Url,
    title: RequiredText,
    summary: RequiredText,
    status: ModerationStatus,
    created_at: Date,
    thumbnail: Image,
    sections: Vec<ArticleSection>,
    category: Category,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Category {
    id: Identifier,
    title: RequiredText,
    emoji: RequiredText,
    status: ModerationStatus,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::common::ImageBuilder;

    #[test]
    fn serialization_succeeds_for_article() {
        let article = article_fixture();
        let serialized = serde_json::json!(&article).to_string();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("Published"));
        assert!(serialized.contains("Dummy title"));
    }

    #[test]
    fn deserialization_succeeds_for_article() {
        let json_str = json_article_fixture();
        let a = serde_json::from_str::<Article>(json_str).unwrap();

        assert_eq!(a.id().to_string(), "413b8ba1-2bc4-4fda-8455-0c0dea528ca0");
        assert_eq!(a.title().to_string(), "Dummy title");
        assert_eq!(a.status().clone(), ModerationStatus::Published);
    }

    fn article_fixture() -> Article {
        ArticleBuilder::default()
            .id(Identifier::try_from("413b8ba1-2bc4-4fda-8455-0c0dea528ca0").unwrap())
            .slug(Url::try_from("/dummy-article").unwrap())
            .title(RequiredText::try_from("Dummy title").unwrap())
            .summary(RequiredText::try_from("Dummy description").unwrap())
            .status(ModerationStatus::Published)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .category(category_fixture())
            .thumbnail(thumbnail_fixture())
            .sections(vec![])
            .build()
            .unwrap()
    }

    fn category_fixture() -> Category {
        CategoryBuilder::default()
            .id(Identifier::try_from("413b8ba1-2bc4-4fda-8455-0c0dea528ca0").unwrap())
            .title(RequiredText::try_from("Example").unwrap())
            .status(ModerationStatus::Published)
            .emoji(RequiredText::try_from("⌨️").unwrap())
            .build()
            .unwrap()
    }

    fn thumbnail_fixture() -> Image {
        ImageBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com/example.png".try_into().unwrap())
            .alt("Dummy image".try_into().unwrap())
            .title("Dummy image".try_into().unwrap())
            .height(500)
            .width(500)
            .build()
            .unwrap()
    }

    fn json_article_fixture() -> &'static str {
        r#"{
            "created_at": "2024-12-15T14:03:56Z",
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "slug": "/dummy-article",
            "sections": [],
            "thumbnail": {
                "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
                "url": "https://example.com/example.png",
                "alt": "Dummy image",
                "title": "Dummy image",
                "height": 500,
                "width": 500
            },
            "status": "Published",
            "title": "Dummy title",
            "summary": "Dummy description",
            "category": {
                "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
                "title": "Example",
                "slug": "/example",
                "status": "Published",
                "emoji": "⌨️"
            }
        }"#
    }
}
