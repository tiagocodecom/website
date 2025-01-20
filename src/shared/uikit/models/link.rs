use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Link {
    url: String,
    title: String,
    options: Vec<(String, String)>,
}
