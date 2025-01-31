use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

pub type MenuItems = Vec<MenuItem>;

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct MenuItem {
    key: String,
    title: String,
    uri: String,
    absolute: String,
    relative: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    weight: isize,
    uuid: Option<String>,
    enabled: bool,
    expanded: bool,
    external: bool,
    field_image: Option<FieldMenuImage>,
}

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct FieldMenuImage {
    field_media_image: Vec<FieldMenuMediaImage>,
}

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct FieldMenuMediaImage {
    target_id: String,
    target_uuid: String,
    target_type: String,
    url: String,
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
    fn deserializes_menu_item_without_image() {
        let json_str = r#"
        {
            "key": "test",
            "title": "test",
            "uri": "test",
            "absolute": "test",
            "relative": "test",
            "weight": "1",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "enabled": true,
            "expanded": false,
            "field_image": null
        }
        "#;

        let menu_item = serde_json::from_str::<MenuItem>(json_str).unwrap();

        assert_eq!(menu_item.key(), "test");
        assert_eq!(menu_item.title(), "test");
        assert_eq!(menu_item.uri(), "test");
        assert_eq!(menu_item.absolute(), "test");
        assert_eq!(menu_item.relative(), "test");
        assert_eq!(menu_item.weight().clone(), 1);
        assert!(menu_item.field_image().is_none());
        assert_eq!(menu_item.enabled().clone(), true);
        assert_eq!(menu_item.expanded().clone(), false);
        assert_eq!(
            *menu_item.uuid(),
            Some("550e8400-e29b-41d4-a716-446655440000".into())
        );
    }

    #[test]
    fn deserializes_menu_item_with_image() {
        let json_str = r#"
        {
            "key": "test",
            "title": "test",
            "uri": "test",
            "absolute": "test",
            "relative": "test",
            "weight": "1",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "enabled": true,
            "expanded": false,
            "field_image": {
                "field_media_image": [{
                    "target_id": "1",
                    "target_uuid": "550e8400-e29b-41d4-a716-446655440000",
                    "target_type": "image",
                    "url": "https://example.com/image.jpg",
                    "alt": "test",
                    "title": "test",
                    "width": "100",
                    "height": "100"
                }]
            }
        }
        "#;
        let menu_item = serde_json::from_str::<MenuItem>(json_str).unwrap();

        assert_eq!(menu_item.key(), "test");
        assert_eq!(menu_item.title(), "test");
        assert_eq!(menu_item.uri(), "test");
        assert_eq!(menu_item.absolute(), "test");
        assert_eq!(menu_item.relative(), "test");
        assert_eq!(*menu_item.weight(), 1);
        assert!(menu_item.field_image().is_some());
        assert_eq!(*menu_item.enabled(), true);
        assert_eq!(*menu_item.expanded(), false);
        assert_eq!(
            menu_item
                .field_image()
                .as_ref()
                .unwrap()
                .field_media_image()
                .len(),
            1
        );
    }
}
