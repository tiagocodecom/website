use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::shared::ui::models::{Document, Image, Link, Timeline};

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Portfolio {
    id: String,
    status: bool,
    title: String,
    created_at: String,
    sections: Vec<Section>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Section {
    AboutMe(AboutMe),
    Resume(Resume),
    Projects(Projects),
    Blogs(Blogs),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Content {
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Resume {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    education: Timeline,
    experience: Timeline,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct AboutMe {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    skills: Vec<String>,
    years_of_experience: u8,
    cv_document: Document,
    profile_picture: Image,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Projects {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Project {
    id: String,
    title: String,
    text: String,
    link: Link,
    image: Image,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Getters)]
pub struct Blogs {
    id: String,
    title: String,
    subtitle: String,
    text: String,
}
