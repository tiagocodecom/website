use crate::application::domain::article::Articles;
use crate::application::domain::common::{Document, Image, Project, Timeline};
use crate::application::value_objects::{Identifier, RequiredText};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Different sections that can be part of a portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortfolioSection {
    AboutMe(AboutMe),
    Resume(Resume),
    Projects(Projects),
    Blogs(Blogs),
    Unknown,
}

/// Resume section containing education and work experience timelines
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Resume {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: RequiredText,
    education: Timeline,
    experience: Timeline,
}

/// About Me section with personal information and skills
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct AboutMe {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: RequiredText,
    skills: Vec<String>,
    cv_document: Document,
    profile_picture: Image,
    years_of_experience: u8,
}

/// Projects section containing a collection of individual projects
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Projects {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: RequiredText,
    projects: Vec<Project>,
}

/// Blog posts section
#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Blogs {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: String,
    articles: Articles,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::common::{DocumentBuilder, ImageBuilder, TimelineBuilder};

    #[test]
    fn serialization_succeeds_for_resume() {
        let resume = resume_fixture();
        let serialized = serde_json::to_string(&resume).unwrap();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("Example title"));
        assert!(serialized.contains("Example subtitle"));
        assert!(serialized.contains("Example text"));
        assert!(serialized.contains("[]"));
    }

    #[test]
    fn serialization_succeeds_for_about_me() {
        let about_me = about_me_fixture();
        let serialized = serde_json::to_string(&about_me).unwrap();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("Example title"));
        assert!(serialized.contains("Example subtitle"));
        assert!(serialized.contains("Example text"));
        assert!(serialized.contains("Example skill 1"));
        assert!(serialized.contains("https://example.com/example.pdf"));
        assert!(serialized.contains("application/pdf"));
        assert!(serialized.contains("https://example.com/example.png"));
        assert!(serialized.contains("Dummy image"));
        assert!(serialized.contains("5"));
    }

    #[test]
    fn serialization_succeeds_for_projects() {
        let projects = projects_fixture();
        let serialized = serde_json::to_string(&projects).unwrap();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("Example title"));
        assert!(serialized.contains("Example subtitle"));
        assert!(serialized.contains("Example text"));
        assert!(serialized.contains("[]"));
    }

    #[test]
    fn serialization_succeeds_for_blogs() {
        let blogs = blogs_fixture();
        let serialized = serde_json::to_string(&blogs).unwrap();

        assert!(serialized.contains("413b8ba1-2bc4-4fda-8455-0c0dea528ca0"));
        assert!(serialized.contains("Example title"));
        assert!(serialized.contains("Example subtitle"));
        assert!(serialized.contains("Example text"));
    }

    #[test]
    fn deserialization_succeeds_for_about_me() {
        let json_str = json_about_me_fixture();
        let about_me: AboutMe = serde_json::from_str(&json_str).unwrap();
        let profile_picture = about_me.profile_picture().url().as_str();
        let cv_document = about_me.cv_document().url().as_str();

        assert_eq!(about_me.title().to_string(), "Example title");
        assert_eq!(about_me.subtitle().to_string(), "Example subtitle");
        assert_eq!(about_me.text().to_string(), "Example text");
        assert_eq!(about_me.skills(), &["Example skill 1".to_string()]);
        assert_eq!(cv_document, "https://example.com/example.pdf");
        assert_eq!(about_me.cv_document().mime().to_string(), "application/pdf");
        assert_eq!(profile_picture, "https://example.com/example.png");
        assert_eq!(about_me.profile_picture().alt().to_string(), "Dummy image");
        assert_eq!(*about_me.profile_picture().width(), 500);
        assert_eq!(*about_me.years_of_experience(), 5);
    }

    #[test]
    fn deserialization_succeeds_for_projects() {
        let json_str = json_projects_fixture();
        let projects: Projects = serde_json::from_str(&json_str).unwrap();

        assert_eq!(projects.title().to_string(), "Example title");
        assert_eq!(projects.subtitle().to_string(), "Example subtitle");
        assert_eq!(projects.text().to_string(), "Example text");
        assert!(projects.projects().is_empty());
    }

    #[test]
    fn deserialization_succeeds_for_resume() {
        let json_str = json_resume_fixture();
        let resume: Resume = serde_json::from_str(&json_str).unwrap();

        assert_eq!(resume.title().to_string(), "Example title");
        assert_eq!(resume.subtitle().to_string(), "Example subtitle");
        assert_eq!(resume.text().to_string(), "Example text");
        assert!(resume.education().items().is_empty());
        assert!(resume.experience().items().is_empty());
    }

    #[test]
    fn deserialization_succeeds_for_blogs() {
        let json_str = json_blogs_fixture();
        let blogs: Blogs = serde_json::from_str(&json_str).unwrap();

        assert_eq!(blogs.title().to_string(), "Example title");
        assert_eq!(blogs.subtitle().to_string(), "Example subtitle");
        assert_eq!(blogs.text().to_string(), "Example text");
    }

    fn resume_fixture() -> Resume {
        ResumeBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text("Example text".try_into().unwrap())
            .education(timeline_fixture())
            .experience(timeline_fixture())
            .build()
            .unwrap()
    }

    fn about_me_fixture() -> AboutMe {
        AboutMeBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text("Example text".try_into().unwrap())
            .skills(vec!["Example skill 1".to_string()])
            .cv_document(document_fixture())
            .profile_picture(image_fixture())
            .years_of_experience(5)
            .build()
            .unwrap()
    }

    fn projects_fixture() -> Projects {
        ProjectsBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text("Example text".try_into().unwrap())
            .projects(vec![])
            .build()
            .unwrap()
    }

    fn blogs_fixture() -> Blogs {
        BlogsBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .articles(vec![])
            .text("Example text".to_string())
            .build()
            .unwrap()
    }

    fn timeline_fixture() -> Timeline {
        TimelineBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .items(vec![])
            .build()
            .unwrap()
    }

    fn document_fixture() -> Document {
        DocumentBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com/example.pdf".try_into().unwrap())
            .mime("application/pdf".try_into().unwrap())
            .build()
            .unwrap()
    }

    fn image_fixture() -> Image {
        ImageBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com/example.png".try_into().unwrap())
            .alt("Dummy image".try_into().unwrap())
            .title("Dummy image".try_into().unwrap())
            .width(500)
            .height(500)
            .build()
            .unwrap()
    }

    fn json_resume_fixture() -> String {
        format!(
            r#"{{
                "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
                "title": "Example title",
                "subtitle": "Example subtitle",
                "text": "Example text",
                "education": {},
                "experience": {}
            }}"#,
            json_timeline_fixture(),
            json_timeline_fixture()
        )
    }

    fn json_about_me_fixture() -> String {
        format!(
            r#"{{
                "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
                "title": "Example title",
                "subtitle": "Example subtitle",
                "text": "Example text",
                "skills": ["Example skill 1"],
                "cv_document": {},
                "profile_picture": {},
                "years_of_experience": 5
            }}"#,
            json_document_fixture(),
            json_image_fixture()
        )
    }

    fn json_projects_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "title": "Example title",
            "subtitle": "Example subtitle",
            "text": "Example text",
            "projects": []
        }"#
    }

    fn json_blogs_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "title": "Example title",
            "subtitle": "Example subtitle",
            "text": "Example text",
            "articles": []
        }"#
    }

    fn json_document_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "url": "https://example.com/example.pdf",
            "mime": "application/pdf"
        }"#
    }

    fn json_image_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "url": "https://example.com/example.png",
            "alt": "Dummy image",
            "title": "Dummy image",
            "width": 500,
            "height": 500
        }"#
    }

    fn json_timeline_fixture() -> &'static str {
        r#"{
            "id": "413b8ba1-2bc4-4fda-8455-0c0dea528ca0",
            "items": []
        }"#
    }
}
