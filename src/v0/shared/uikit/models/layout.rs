use crate::shared::uikit::models::{Image, Menu};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, Builder, Clone, Debug)]
pub struct Layout {
    pub logo: Image,
    pub main_menu: Menu,
    pub social_menu: Menu,
}
