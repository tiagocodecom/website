use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct FieldImage {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    name: String,
    langcode: String,
    media_image: MediaImage,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct MediaImage {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    image_style_uri: ImageStyleUri,
    meta: Meta,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct ImageStyleUri {
    max_2600x2600: String,
    medium_500x500: String,
    medium: String,
    thumbnail: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct Meta {
    alt: String,
    title: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    width: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    height: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_field_image_response() {
        let json_str = mock_field_image_response();
        let field_image = serde_json::from_str::<FieldImage>(json_str).unwrap();

        assert_eq!(field_image.id(), "3493a08d-f2f2-4148-9541-01dbf619a19b");
        assert_eq!(field_image.name(), "example.jpg");
        assert_eq!(field_image.langcode(), "en");

        let media_image = field_image.media_image();

        assert_eq!(media_image.id(), "ad19e979-e05c-48a1-82b4-6b9d8e415661");
        assert_eq!(media_image.meta().alt(), "Example");
        assert_eq!(media_image.meta().title(), "");
        assert_eq!(*media_image.meta().width(), 500);
        assert_eq!(*media_image.meta().height(), 500);

        let style = media_image.image_style_uri();

        assert_eq!(
            style.max_2600x2600(),
            "https://example.com/styles/max_2600x2600/public/2025-01/example.jpg.webp"
        );
        assert_eq!(
            style.medium_500x500(),
            "https://example.com/styles/medium_500x500/public/2025-01/example.jpg"
        );
        assert_eq!(
            style.medium(),
            "https://example.com/styles/medium/public/2025-01/example.jpg.webp"
        );
        assert_eq!(
            style.thumbnail(),
            "https://example.com/styles/thumbnail/public/2025-01/example.jpg.webp"
        );
    }

    fn mock_field_image_response() -> &'static str {
        r#"
        {
            "type": "media--image",
            "id": "3493a08d-f2f2-4148-9541-01dbf619a19b",
            "langcode": "en",
            "name": "example.jpg",
            "media_image": {
                "type": "file--file",
                "id": "ad19e979-e05c-48a1-82b4-6b9d8e415661",
                "uri": {
                    "value": "public://2025-01/example.jpg",
                    "url": "/files/2025-01/example.jpg"
                },
                "filemime": "image/jpeg",
                "filesize": "57491",
                "image_style_uri": {
                    "medium_500x500": "https://example.com/styles/medium_500x500/public/2025-01/example.jpg",
                    "max_2600x2600": "https://example.com/styles/max_2600x2600/public/2025-01/example.jpg.webp",
                    "medium": "https://example.com/styles/medium/public/2025-01/example.jpg.webp",
                    "thumbnail": "https://example.com/styles/thumbnail/public/2025-01/example.jpg.webp"
                },
                "meta": {
                    "alt": "Example",
                    "title": "",
                    "width": "500",
                    "height": "500"
                }
            }
        }
        "#
    }
}
