//! Portfolio Paragraph Models
//!
//! This module provides models for portfolio-specific paragraphs that represent
//! different sections of a portfolio page, such as "About Me", "Resume", and
//! "Projects" sections.
//!
//! Portfolio paragraphs differ from content paragraphs in that they are specific
//! to portfolio pages and define the high-level structure, while content paragraphs
//! are reusable components that can appear within these sections.
//!
//! These models represent the structure of portfolio sections as returned by the API.
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::cms_content::adapters::output::api::models::{FieldContent, FieldDocument, FieldImage};

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ParagraphPortfolioResume {
    id: String,
    subtitle: String,
    text: String,
    title: String,
    items: Vec<FieldContent>,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ParagraphPortfolioAboutMe {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    text_list: Vec<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    years_of_experience: u8,
    image: FieldImage,
    document: FieldDocument,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ParagraphPortfolioProjects {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    items: Vec<FieldContent>,
}
