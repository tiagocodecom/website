use leptos::prelude::*;

use crate::cms_content::domain::common::{Document, Image};
use crate::portfolio::components::Badge;
use crate::uikit::components::*;

#[component]
pub fn AboutMeSection(
    subtitle: String,
    title: String,
    text: String,
    skills: Vec<String>,
    profile_picture: Image,
    years_of_experience: u8,
    cv_document: Document,
) -> impl IntoView {
    view! {
        <Container id="about">
            <div class="lg:flex space-y-5 lg:space-x-10">
                <div>
                    <div class="flex justify-center relative h-fit">
                        <img
                            class="min-w-52 min-h-52 max-w-64 max-h-64 rounded-full"
                            src=profile_picture.url().clone()
                            alt=profile_picture.alt().clone()
                        />
                    </div>
                    <div class="flex items-center justify-center">
                        <div class="pe-2">
                            <div class="font-mono font-semibold text-6xl stroke-text">
                                <span class="counter">{years_of_experience}</span>
                            </div>
                        </div>
                        <div class="dark:text-white py-2">
                            <span class="block text-2xl font-normal mb-1">+</span>
                            <p class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Years of Experience</p>
                        </div>
                    </div>
                </div>
                <div>
                    <ContainerSubtitle backdrop_text="About me">{subtitle}</ContainerSubtitle>
                    <ContainerTitle>{title}</ContainerTitle>
                    <ContainerDescription>{text}</ContainerDescription>
                    <div class="space-y-3 mb-2">
                        {skills.into_iter().map(|s| view! { <Badge>{s}</Badge> }).collect_view()}
                    </div>
                    <div class="space-y-3 mb-2 flex justify-end">
                        <a href=cv_document.url().clone() class="btn btn-primary" target="_blank">
                            <span class="btn-text">"Download cv"</span>
                            <i class="bi bi-cloud-download ps-1"></i>
                        </a>
                    </div>
                </div>
            </div>
        </Container>
    }
}
