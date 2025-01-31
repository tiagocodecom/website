use super::models::MenuBuilder;
use crate::shared::api::models::MenuItem as ApiMenuItem;
use crate::shared::uikit::models::{ImageBuilder, Menu, MenuItemBuilder};
use crate::shared::uikit::models::{Layout, LayoutBuilder};
use itertools::Itertools;

pub fn layout_adapter(social_menu: Vec<ApiMenuItem>, main_menu: Vec<ApiMenuItem>) -> Layout {
    LayoutBuilder::default()
        .logo(
            ImageBuilder::default()
                .width(100)
                .height(100)
                .id("logo".to_string())
                .alt("Tiagocode logo".to_string())
                .title("Tiagocode logo".to_string())
                .url("/assets/images/logo.png".to_string())
                .build()
                .unwrap(),
        )
        .main_menu(menu_items_adapter("main", main_menu))
        .social_menu(menu_items_adapter("social-network", social_menu))
        .build()
        .unwrap()
}

pub fn menu_items_adapter(name: &str, menu_items: Vec<ApiMenuItem>) -> Menu {
    MenuBuilder::default()
        .name(name.to_string())
        .items(
            menu_items
                .iter()
                .filter(|item| item.enabled().clone())
                .sorted_by(|a, b| a.weight().cmp(b.weight()))
                .map(|item| {
                    MenuItemBuilder::default()
                        .id(item.key().clone())
                        .title(item.title().clone())
                        .url(if item.external().clone() {
                            item.absolute().clone()
                        } else {
                            item.relative().clone()
                        })
                        .show(item.enabled().clone())
                        .weight(item.weight().clone())
                        .icon(
                            item.field_image()
                                .as_ref()
                                .and_then(|field_image| field_image.field_media_image().first())
                                .map(|media_image| {
                                    ImageBuilder::default()
                                        .id(item.key().clone())
                                        .url(media_image.url().clone())
                                        .alt(media_image.alt().clone())
                                        .title(media_image.title().clone())
                                        .width(media_image.width().clone())
                                        .height(media_image.height().clone())
                                        .build()
                                        .unwrap()
                                }),
                        )
                        .build()
                        .unwrap()
                })
                .collect(),
        )
        .build()
        .unwrap()
}
