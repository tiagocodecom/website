use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Timeline {
    id: String,
    items: Vec<TimelineItem>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct TimelineItem {
    id: String,
    date: String,
    title: String,
    subtitle: String,
    text: Option<String>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Image {
    id: String,
    url: String,
    alt: String,
    title: String,
    width: u16,
    height: u16,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Document {
    id: String,
    url: String,
    mime: String,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Link {
    url: String,
    title: String,
    options: Vec<(String, String)>,
}
