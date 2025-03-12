use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::common::Image;
use crate::application::value_objects::{Identifier, Url};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Layout {
    logo: Option<Image>,
    main_menu: MenuTree,
    social_menu: MenuTree,
    sidebar_menu: Option<MenuTree>,
    footer_menu: Option<MenuTree>,
}

#[derive(Serialize, Deserialize, Builder, Getters, Clone, Debug)]
pub struct MenuTree {
    items: Vec<MenuItem>,
}

#[derive(Serialize, Deserialize, Builder, Getters, Clone, Debug)]
pub struct MenuItem {
    id: Identifier,
    url: Url,
    title: String,
    hidden: bool,
    weight: isize,
    icon: Option<Image>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_succeeds_for_menu_item() {
        let menu_item = menu_item_fixture();
        let serialized = serde_json::to_string(&menu_item).unwrap();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("https://example.com"));
        assert!(serialized.contains("Example"));
        assert!(serialized.contains("false"));
        assert!(serialized.contains("0"));
        assert!(serialized.contains("null"));
    }

    #[test]
    fn deserialization_succeeds_for_menu_item() {
        let json_str = json_menu_item_fixture();
        let deserialized: MenuItem = serde_json::from_str(json_str).unwrap();

        assert_eq!(deserialized.url().to_string(), "https://example.com");
        assert_eq!(deserialized.title().to_string(), "Example");
        assert_eq!(*deserialized.hidden(), false);
        assert_eq!(*deserialized.weight(), 0);
    }

    fn menu_item_fixture() -> MenuItem {
        MenuItemBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com".try_into().unwrap())
            .title("Example".into())
            .hidden(false)
            .weight(0)
            .icon(None)
            .build()
            .unwrap()
    }

    fn json_menu_item_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "url": "https://example.com",
            "title": "Example",
            "hidden": false,
            "weight": 0,
            "icon": null
        }"#
    }
}
