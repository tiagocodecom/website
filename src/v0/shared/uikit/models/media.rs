use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Image {
    id: String,
    url: String,
    alt: String,
    title: String,
    width: u16,
    height: u16,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Document {
    id: String,
    url: String,
    mime: String,
}
