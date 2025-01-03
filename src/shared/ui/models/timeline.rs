use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Timeline {
    id: String,
    elements: Vec<TimelineItem>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct TimelineItem {
    id: String,
    date: String,
    title: String,
    subtitle: String,
    text: Option<String>,
}
