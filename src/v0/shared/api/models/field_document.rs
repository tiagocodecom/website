//! Media Image Models
//!
//! This module provides models for handling media image responses from the API.
//! It defines the structure for image resources, including metadata and different
//! image style variations (e.g. thumbnails, medium sizes).
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct FieldDocument {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    name: String,
    langcode: String,
    media_document: MediaDocument,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct MediaDocument {
    id: String,
    uri: Uri,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(rename = "filemime")]
    mime_type: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    filesize: u64,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct Uri {
    url: String,
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_field_document() {
        let json_str = mock_field_document();
        let field_document = serde_json::from_str::<FieldDocument>(json_str).unwrap();

        assert_eq!(field_document.langcode(), "en");
        assert_eq!(field_document.name(), "example.pdf");
        assert_eq!(field_document.entity_type(), "media--document");
        assert_eq!(field_document.id(), "e4c3f6db-fa69-4d4d-a006-98b8a7763924");

        let media_document = field_document.media_document();

        assert_eq!(*media_document.filesize(), 106995);
        assert_eq!(media_document.mime_type(), "application/pdf");
        assert_eq!(media_document.id(), "ae475422-f200-4d50-9b1d-016fc38b3047");
        assert_eq!(media_document.uri().value(), "public://2025-01/example.pdf");
        assert_eq!(
            media_document.uri().url(),
            "/sites/default/files/2025-01/example.pdf"
        );
    }

    fn mock_field_document() -> &'static str {
        r#"
        {
            "type": "media--document",
            "id": "e4c3f6db-fa69-4d4d-a006-98b8a7763924",
            "langcode": "en",
            "name": "example.pdf",
            "meta": {
                "drupal_internal__target_id": "10"
            },
            "media_document": {
                "type": "file--file",
                "id": "ae475422-f200-4d50-9b1d-016fc38b3047",
                "uri": {
                    "value": "public://2025-01/example.pdf",
                    "url": "/sites/default/files/2025-01/example.pdf"
                },
                "filemime": "application/pdf",
                "filesize": "106995",
                "image_style_uri": null,
                "meta": {
                    "display": "",
                    "description": "",
                    "drupal_internal__target_id": "13"
                }
            }
        }
        "#
    }
}
