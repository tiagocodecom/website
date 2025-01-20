use crate::shared::uikit::models::Image;
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Builder, Getters, Clone, Debug)]
pub struct Menu {
    name: String,
    items: Vec<MenuItem>,
}

#[derive(Serialize, Deserialize, Builder, Getters, Clone, Debug)]
pub struct MenuItem {
    id: String,
    title: String,
    url: String,
    show: bool,
    weight: isize,
    icon: Option<Image>,
}
