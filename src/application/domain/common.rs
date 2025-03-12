use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::value_objects::{Identifier, RequiredText, Url};

/// Individual project with details and media
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Project {
    id: Identifier,
    title: RequiredText,
    text: String,
    link: Link,
    image: Image,
}

/// Timeline containing ordered items
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Timeline {
    id: Identifier,
    items: Vec<TimelineItem>,
}

/// Single timeline entry with date and details
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct TimelineItem {
    id: Identifier,
    date: RequiredText,
    title: RequiredText,
    subtitle: RequiredText,
    text: Option<String>,
}

/// Image asset with metadata
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Image {
    id: Identifier,
    url: Url,
    alt: RequiredText,
    title: RequiredText,
    width: u16,
    height: u16,
}

/// Document file with metadata
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Document {
    id: Identifier,
    url: Url,
    mime: RequiredText,
}

/// Hyperlink with title and optional parameters
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Link {
    url: Url,
    title: String,
    options: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_succeeds_for_timeline_item() {
        let timeline_item = timeline_item_fixture();

        assert_eq!(timeline_item.title().to_string(), "Example title");
    }

    #[test]
    fn builder_succeeds_for_project() {
        let project = project_fixture();

        assert_eq!(project.title().to_string(), "Example project");
        assert_eq!(project.text(), "Example project description");
        assert_eq!(project.link().url().to_string(), "https://example.com");
    }

    #[test]
    fn builder_succeeds_for_image() {
        let image = image_fixture();

        assert_eq!(image.alt().to_string(), "Dummy image");
        assert_eq!(image.title().to_string(), "Dummy image");
        assert_eq!(*image.width(), 500);
        assert_eq!(*image.height(), 500);
    }

    #[test]
    fn builder_succeeds_for_document() {
        let document = document_fixture();

        assert_eq!(document.url().as_str(), "https://example.com/example.pdf");
        assert_eq!(document.mime().to_string(), "application/pdf");
    }

    #[test]
    fn builder_succeeds_for_link() {
        let link = link_fixture();

        assert_eq!(link.url().to_string(), "https://example.com");
        assert_eq!(link.title().to_string(), "Example link");
    }

    #[test]
    fn serialization_succeeds_for_timeline_item() {
        let timeline_item = timeline_item_fixture();
        let serialized = serde_json::json!(&timeline_item).to_string();

        assert!(serialized.contains("2024-12-15 14:03:56"));
        assert!(serialized.contains("Example title"));
        assert!(serialized.contains("Example subtitle"));
    }

    #[test]
    fn serialization_succeeds_for_project() {
        let project = project_fixture();
        let serialized = serde_json::json!(&project).to_string();

        assert!(serialized.contains("Example project"));
        assert!(serialized.contains("Example project description"));
    }

    #[test]
    fn serialization_succeeds_for_image() {
        let image = image_fixture();
        let serialized = serde_json::json!(&image).to_string();

        assert!(serialized.contains(r#"https://example.com/example.png"#));
        assert!(serialized.contains(r#"Dummy image"#));
    }

    #[test]
    fn serialization_succeeds_for_document() {
        let document = document_fixture();
        let serialized = serde_json::json!(&document).to_string();

        assert!(serialized.contains(r#"https://example.com/example.pdf"#));
        assert!(serialized.contains(r#"application/pdf"#));
    }

    #[test]
    fn serialization_succeeds_for_link() {
        let link = link_fixture();
        let serialized = serde_json::json!(&link).to_string();

        assert!(serialized.contains("https://example.com"));
        assert!(serialized.contains("Example link"));
    }

    #[test]
    fn deserialization_succeeds_for_project() {
        let json_str = json_project_fixture();
        let project: Project = serde_json::from_str(json_str).unwrap();

        assert_eq!(project.title().to_string(), "Example project");
        assert_eq!(project.text(), "Example project description");
        assert_eq!(project.link().url().to_string(), "https://example.com");
    }

    #[test]
    fn deserialization_succeeds_for_timeline_item() {
        let json_str = json_timeline_item_fixture();
        let timeline_item: TimelineItem = serde_json::from_str(json_str).unwrap();

        assert_eq!(timeline_item.date().to_string(), "2024-12-15 14:03:56");
        assert_eq!(timeline_item.title().to_string(), "Example title");
        assert_eq!(timeline_item.title().to_string(), "Example title");
        assert_eq!(timeline_item.title().to_string(), "Example title");
        assert_eq!(timeline_item.title().to_string(), "Example title");
        assert_eq!(timeline_item.subtitle().to_string(), "Example subtitle");
    }

    #[test]
    fn deserialization_succeeds_for_document() {
        let json_str = json_document_fixture();
        let document: Document = serde_json::from_str(json_str).unwrap();

        assert_eq!(document.url().as_str(), "https://example.com/example.pdf");
        assert_eq!(document.mime().to_string(), "application/pdf");
    }

    #[test]
    fn deserialization_succeeds_for_image() {
        let json_str = json_image_fixture();
        let image: Image = serde_json::from_str(json_str).unwrap();

        assert_eq!(image.url().as_str(), "https://example.com/example.png");
        assert_eq!(image.alt().to_string(), "Dummy image");
        assert_eq!(image.title().to_string(), "Dummy image");
        assert_eq!(*image.width(), 500);
        assert_eq!(*image.height(), 500);
    }

    #[test]
    fn deserialization_succeeds_for_link() {
        let json_str = json_link_fixture();
        let link: Link = serde_json::from_str(json_str).unwrap();

        assert_eq!(link.url().to_string(), "https://example.com");
        assert_eq!(link.title().to_string(), "Example link");
    }

    fn link_fixture() -> Link {
        LinkBuilder::default()
            .url("https://example.com".try_into().unwrap())
            .title("Example link".to_string())
            .options(vec![])
            .build()
            .unwrap()
    }

    fn document_fixture() -> Document {
        DocumentBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com/example.pdf".try_into().unwrap())
            .mime("application/pdf".try_into().unwrap())
            .build()
            .unwrap()
    }

    fn image_fixture() -> Image {
        ImageBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com/example.png".try_into().unwrap())
            .alt("Dummy image".try_into().unwrap())
            .title("Dummy image".try_into().unwrap())
            .width(500)
            .height(500)
            .build()
            .unwrap()
    }

    fn project_fixture() -> Project {
        ProjectBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .title("Example project".try_into().unwrap())
            .text("Example project description".to_string())
            .link(link_fixture())
            .image(image_fixture())
            .build()
            .unwrap()
    }

    fn timeline_item_fixture() -> TimelineItem {
        TimelineItemBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .date("2024-12-15 14:03:56".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text(Some("Example text".to_string()))
            .build()
            .unwrap()
    }

    fn json_link_fixture() -> &'static str {
        r#"{
            "title": "Example link",
            "url": "https://example.com",
            "options": []
        }"#
    }

    fn json_document_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "url": "https://example.com/example.pdf",
            "mime": "application/pdf"
        }"#
    }

    fn json_image_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "url": "https://example.com/example.png",
            "alt": "Dummy image",
            "title": "Dummy image",
            "width": 500,
            "height": 500
        }"#
    }

    fn json_project_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "title": "Example project",
            "text": "Example project description",
            "link": {
                "title": "Example link",
                "url": "https://example.com",
                "options": []
            },
            "image": {
                "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
                "url": "https://example.com/example.png",
                "alt": "Dummy image",
                "title": "Dummy image",
                "width": 500,
                "height": 500
            }
        }"#
    }

    fn json_timeline_item_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "date": "2024-12-15 14:03:56",
            "title": "Example title",
            "subtitle": "Example subtitle",
            "text": "Example text"
        }"#
    }
}
