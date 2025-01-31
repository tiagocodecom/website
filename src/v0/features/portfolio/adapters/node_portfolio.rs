use crate::features::portfolio::models::ProjectBuilder;
use crate::features::portfolio::models::{AboutMeBuilder, PortfolioBuilder, Project};
use crate::features::portfolio::models::{Portfolio, ProjectsBuilder, ResumeBuilder, Section};
use crate::shared::api::models::{FieldContent, NodePortfolio};
use crate::shared::api::models::{FieldDocument, FieldImage, Resource};
use crate::shared::uikit::models::{Document, DocumentBuilder, Image};
use crate::shared::uikit::models::{ImageBuilder, LinkBuilder, Timeline};
use crate::shared::uikit::models::{TimelineBuilder, TimelineItemBuilder};
use voca_rs::strip::strip_tags;

pub fn adapt_node_portfolio(document: Resource<NodePortfolio>) -> Result<Portfolio, String> {
    let portfolio = PortfolioBuilder::default()
        .id(document.data().id().to_string())
        .status(document.data().status().clone())
        .title(document.data().title().to_string())
        .created_at(document.data().created_at().to_string())
        .sections(
            document
                .data()
                .content()
                .iter()
                .map(|content| match content {
                    FieldContent::PortfolioAboutMe(_) => adapt_portfolio_about_me(content),
                    FieldContent::PortfolioResume(_) => adapt_portfolio_resume(content),
                    FieldContent::PortfolioProjects(_) => adapt_portfolio_projects(content),
                    _ => Section::Unknown,
                })
                .collect(),
        )
        .build()
        .unwrap();

    Ok(portfolio)
}

fn adapt_portfolio_about_me(p: &FieldContent) -> Section {
    if let FieldContent::PortfolioAboutMe(p) = p {
        return Section::AboutMe(
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
    Section::Unknown
}

fn adapt_portfolio_resume(p: &FieldContent) -> Section {
    if let FieldContent::PortfolioResume(p) = p {
        return Section::Resume(
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
    Section::Unknown
}

fn adapt_portfolio_projects(p: &FieldContent) -> Section {
    if let FieldContent::PortfolioProjects(p) = p {
        return Section::Projects(
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
    Section::Unknown
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
            .elements(
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
