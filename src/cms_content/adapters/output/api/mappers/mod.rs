use voca_rs::strip::strip_tags;

use crate::cms_content::adapters::output::api::models::{
    FieldContent, FieldDocument, FieldImage, NodePortfolio, Resource,
};
use crate::cms_content::application::ports::output::LoadPortfolioStateMapper;
use crate::cms_content::domain::common::{
    Document, DocumentBuilder, Image, ImageBuilder, LinkBuilder, Timeline, TimelineBuilder,
    TimelineItemBuilder,
};
use crate::cms_content::domain::error::{CmsError, CmsResult};
use crate::cms_content::domain::portfolio::{
    AboutMeBuilder, Portfolio, PortfolioBuilder, PortfolioSection, Project, ProjectBuilder,
    ProjectsBuilder, ResumeBuilder,
};

#[derive(Default)]
pub struct CmsPortfolioMapper;

impl LoadPortfolioStateMapper for CmsPortfolioMapper {
    type Input = Resource<NodePortfolio>;

    fn transform(&self, input: Self::Input) -> CmsResult<Portfolio> {
        map_external_portfolio(input).map_err(|e| CmsError::Unknown(e.to_string()))
    }
}

pub fn map_external_portfolio(document: Resource<NodePortfolio>) -> Result<Portfolio, String> {
    let sections = document
        .data()
        .content()
        .iter()
        .map(|content| match content {
            FieldContent::PortfolioResume(_) => map_resume_section(&content),
            FieldContent::PortfolioAboutMe(_) => map_about_me_section(&content),
            FieldContent::PortfolioProjects(_) => map_projects_section(&content),
            _ => PortfolioSection::Unknown,
        })
        .collect();

    let portfolio = PortfolioBuilder::default()
        .id(document.data().id().to_string())
        .status(document.data().status().clone())
        .title(document.data().title().to_string())
        .created_at(document.data().created_at().to_string())
        .sections(sections)
        .build()
        .unwrap();

    Ok(portfolio)
}

fn map_about_me_section(p: &FieldContent) -> PortfolioSection {
    if let FieldContent::PortfolioAboutMe(p) = p {
        return PortfolioSection::AboutMe(
            AboutMeBuilder::default()
                .id(p.id().to_string())
                .title(strip_tags(p.title()))
                .subtitle(strip_tags(p.subtitle()))
                .text(strip_tags(p.text()))
                .skills(p.text_list().to_vec())
                .years_of_experience(p.years_of_experience().clone())
                .profile_picture(adapt_field_image(p.image()))
                .cv_document(adapt_field_document(p.document()))
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn map_resume_section(p: &FieldContent) -> PortfolioSection {
    if let FieldContent::PortfolioResume(p) = p {
        return PortfolioSection::Resume(
            ResumeBuilder::default()
                .id(p.id().clone())
                .title(strip_tags(p.title()))
                .subtitle(strip_tags(p.subtitle()))
                .text(strip_tags(p.text()))
                .education(p.items().iter().nth(0).map(adapt_timeline).unwrap())
                .experience(p.items().iter().nth(1).map(adapt_timeline).unwrap())
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn map_projects_section(p: &FieldContent) -> PortfolioSection {
    if let FieldContent::PortfolioProjects(p) = p {
        return PortfolioSection::Projects(
            ProjectsBuilder::default()
                .id(p.id().to_string())
                .title(strip_tags(p.title()))
                .subtitle(strip_tags(p.subtitle()))
                .text(strip_tags(p.text()))
                .projects(p.items().iter().map(adapt_project).collect())
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn adapt_field_image(p: &FieldImage) -> Image {
    let image_url = p
        .media_image()
        .image_style_uri()
        .medium_500x500()
        .to_string();

    ImageBuilder::default()
        .id(p.id().to_string())
        .title(p.media_image().meta().title().to_string())
        .alt(p.media_image().meta().alt().to_string())
        .height(p.media_image().meta().height().clone())
        .width(p.media_image().meta().width().clone())
        .url(image_url)
        .build()
        .unwrap()
}

fn adapt_field_document(p: &FieldDocument) -> Document {
    DocumentBuilder::default()
        .id(p.id().to_string())
        .url(p.media_document().uri().url().to_string())
        .mime(p.media_document().mime_type().to_string())
        .build()
        .unwrap()
}

fn adapt_timeline(s: &FieldContent) -> Timeline {
    if let FieldContent::ContentTimeline(p) = s {
        return TimelineBuilder::default()
            .id(p.id().to_string())
            .items(
                p.items()
                    .iter()
                    .map(|s| {
                        TimelineItemBuilder::default()
                            .id(s.id().to_string())
                            .date(s.date4human().to_string())
                            .title(s.title().to_string())
                            .subtitle(s.subtitle().to_string())
                            .text(s.text().clone())
                            .build()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
            .build()
            .unwrap();
    }
    TimelineBuilder::default().build().unwrap()
}

fn adapt_project(s: &FieldContent) -> Project {
    if let FieldContent::ContentProject(p) = s {
        return ProjectBuilder::default()
            .id(p.id().to_string())
            .title(strip_tags(p.title()))
            .text(strip_tags(p.text().clone().unwrap_or_default().as_str()))
            .link(
                LinkBuilder::default()
                    .url(p.link().uri().to_string())
                    .title(p.link().title().to_string())
                    .options(vec![])
                    .build()
                    .unwrap(),
            )
            .image(adapt_field_image(p.media()))
            .build()
            .unwrap();
    }
    ProjectBuilder::default().build().unwrap()
}
