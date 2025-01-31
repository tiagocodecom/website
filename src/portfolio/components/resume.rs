use leptos::prelude::*;

use crate::cms_content::domain::common::Timeline;
use crate::portfolio::components::{Timeline, TimelineItem};
use crate::uikit::components::*;

#[component]
pub fn ResumeSection(
    title: String,
    subtitle: String,
    text: String,
    education: Timeline,
    experience: Timeline,
) -> impl IntoView {
    view! {
        <Container id="resume">
            <div class="">
                <ContainerSubtitle backdrop_text="Resume">{subtitle}</ContainerSubtitle>
                <ContainerTitle>{title}</ContainerTitle>
                <ContainerDescription>{text}</ContainerDescription>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-8 lg:gap-10 mt-6 lg:mt-12">
                <Timeline icon="bi bi-mortarboard">
                    {education
                        .items()
                        .into_iter()
                        .map(|e| {
                            view! {
                                <TimelineItem
                                    date=e.date().clone()
                                    title=e.title().clone()
                                    subtitle=e.subtitle().clone()
                                />
                            }
                        })
                        .collect_view()}
                </Timeline>
                <Timeline icon="bi bi-briefcase">
                    {experience
                        .items()
                        .into_iter()
                        .map(|e| {
                            view! {
                                <TimelineItem
                                    date=e.date().clone()
                                    title=e.title().clone()
                                    subtitle=e.subtitle().clone()
                                />
                            }
                        })
                        .collect_view()}
                </Timeline>
            </div>
        </Container>
    }
}
