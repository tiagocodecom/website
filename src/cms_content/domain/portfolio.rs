use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::common::{Document, Image, Link, Timeline};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Portfolio {
    id: String,
    status: bool,
    title: String,
    created_at: String,
    sections: Vec<PortfolioSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortfolioSection {
    AboutMe(AboutMe),
    Resume(Resume),
    Projects(Projects),
    Blogs(Blogs),
    Unknown,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Resume {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    education: Timeline,
    experience: Timeline,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct AboutMe {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    skills: Vec<String>,
    cv_document: Document,
    profile_picture: Image,
    years_of_experience: u8,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Projects {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    projects: Vec<Project>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Project {
    id: String,
    title: String,
    text: String,
    link: Link,
    image: Image,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Blogs {
    id: String,
    title: String,
    subtitle: String,
    text: String,
}
