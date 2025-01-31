use crate::shared::api::models::{
    ParagraphContentHoverCard, ParagraphContentTimeline, ParagraphPortfolioAboutMe,
    ParagraphPortfolioProjects, ParagraphPortfolioResume,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum FieldContent {
    #[serde(rename = "paragraph--content_timeline")]
    ContentTimeline(ParagraphContentTimeline),

    #[serde(rename = "paragraph--content_hover_card")]
    ContentProject(ParagraphContentHoverCard),

    #[serde(rename = "paragraph--portfolio_about_me")]
    PortfolioAboutMe(ParagraphPortfolioAboutMe),

    #[serde(rename = "paragraph--portfolio_resume")]
    PortfolioResume(ParagraphPortfolioResume),

    #[serde(rename = "paragraph--portfolio_projects")]
    PortfolioProjects(ParagraphPortfolioProjects),

    #[serde(other)]
    Unknown,
}
