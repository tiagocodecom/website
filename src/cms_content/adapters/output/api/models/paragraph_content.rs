//! Content Paragraph Models
//!
//! This module provides models for content paragraphs, which represent reusable
//! components that can be used across different sections of the site, such as
//! timelines, accordions, and other global UI elements.
//!
//! Content paragraphs differ from section-specific paragraphs (like portfolio
//! paragraphs) in that they are generic building blocks that can be composed
//! together to create more complex content structures.
//!
//! These models represent the content paragraphs that the API returns.
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::cms_content::adapters::output::api::models::{FieldImage, FieldLink};

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ParagraphContentTimeline {
    id: String,
    items: Vec<ParagraphContentTimelineItem>,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ParagraphContentTimelineItem {
    id: String,
    title: String,
    subtitle: String,
    text: Option<String>,
    #[serde(rename = "date_human_readable")]
    date4human: String,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ParagraphContentHoverCard {
    id: String,
    title: String,
    text: Option<String>,
    link: FieldLink,
    media: FieldImage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_paragraph_content_timeline() {
        let json_str = mock_paragraph_content_timeline();
        let timeline = serde_json::from_str::<ParagraphContentTimeline>(json_str).unwrap();

        assert_eq!(timeline.id(), "d71e003e-699b-48e5-b4b3-19027e7e4699");
        assert_eq!(timeline.items().len(), 2);

        let item = timeline.items().first().unwrap();

        assert_eq!(item.id(), "55c6016c-2fac-4f2c-a4a6-59df6295ed2e");
        assert_eq!(item.title(), "Example");
        assert_eq!(item.subtitle(), "Example");
        assert_eq!(item.date4human(), "Aug 2021");
        assert!(item.text().is_none());

        let item2 = timeline.items().last().unwrap();

        assert_eq!(item2.id(), "ab38be9b-d98f-4cd4-80c6-d8ad4008d2e6");
        assert_eq!(item2.title(), "Example");
        assert_eq!(item2.subtitle(), "Example");
        assert_eq!(item2.date4human(), "Jan 2016 - Dec 2019");
        assert!(item2.text().is_none());
    }

    #[test]
    fn deserializes_paragraph_content_hover_card() {
        let json_str = mock_paragraph_content_hover_card();
        let hover_card = serde_json::from_str::<ParagraphContentHoverCard>(json_str).unwrap();

        assert_eq!(hover_card.id(), "0d2ec132-81d1-44b1-917e-ce889fd22d42");
        assert_eq!(hover_card.title(), "Example");
        assert!(hover_card.text().is_none());
        assert_eq!(hover_card.link().uri(), "https://www.example.com/");
        assert_eq!(hover_card.media().name(), "example.png");
        assert_eq!(hover_card.media().langcode(), "en");

        let media_image = hover_card.media().media_image();

        assert_eq!(media_image.id(), "87c957de-e2f4-48e2-b6f9-2b9981a26e83");
        assert_eq!(media_image.meta().alt(), "Example");
        assert_eq!(media_image.meta().title(), "");
        assert_eq!(*media_image.meta().width(), 500);
        assert_eq!(*media_image.meta().height(), 500);
    }

    fn mock_paragraph_content_timeline() -> &'static str {
        r#"
        {
            "type": "paragraph--content_timeline",
            "id": "d71e003e-699b-48e5-b4b3-19027e7e4699",
            "items": [
              {
                "type": "paragraph--content_timeline_item",
                "id": "55c6016c-2fac-4f2c-a4a6-59df6295ed2e",
                "date_human_readable": "Aug 2021",
                "subtitle": "Example",
                "text": null,
                "title": "Example"
              },
              {
                "type": "paragraph--content_timeline_item",
                "id": "ab38be9b-d98f-4cd4-80c6-d8ad4008d2e6",
                "date_human_readable": "Jan 2016 - Dec 2019",
                "subtitle": "Example",
                "text": null,
                "title": "Example"
              }
            ]
        }"#
    }

    fn mock_paragraph_content_hover_card() -> &'static str {
        r#"
        {
            "type": "paragraph--content_hover_card",
            "id": "0d2ec132-81d1-44b1-917e-ce889fd22d42",
            "link": {
                "uri": "https://www.example.com/",
                "title": "",
                "options": []
            },
            "text": null,
            "title": "Example",
            "media": {
                "type": "media--image",
                "id": "015ad21e-897f-478f-bc07-a980588595ef",
                "langcode": "en",
                "name": "example.png",
                "media_image": {
                    "type": "file--file",
                    "id": "87c957de-e2f4-48e2-b6f9-2b9981a26e83",
                    "uri": {
                        "value": "public://2025-01/example.png",
                        "url": "/2025-01/example.png"
                    },
                    "filemime": "image/png",
                    "filesize": "270871",
                    "image_style_uri": {
                        "medium_500x500": "https://example.com/styles/medium_500x500/public/2025-01/example.png",
                        "max_2600x2600": "https://example.com/styles/max_2600x2600/public/2025-01/example.png.webp",
                        "medium": "https://example.com/styles/medium/public/2025-01/example.png.webp",
                        "thumbnail": "https://example.com/styles/thumbnail/public/2025-01/example.png.webp"
                    },
                    "meta": {
                        "alt": "Example",
                        "title": "",
                        "width": "500",
                        "height": "500"
                    }
                }
            }
        }"#
    }
}
