use leptos::either::EitherOf5;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::controllers::get_portfolio_controller;
use crate::adapters::driver::leptos_webui::views::common::layouts::*;
use crate::adapters::driver::leptos_webui::views::portfolio::components::*;
use crate::application::domain::portfolio_section::PortfolioSection;

#[component]
pub fn PortfolioPage() -> impl IntoView {
    let page_data = OnceResource::new(get_portfolio_controller());

    view! {
        <BasicLayout>
            <Suspense fallback=move || {
                view! { <div>"Loading..."</div> }
            }>
                {move || {
                    page_data
                        .get_untracked()
                        .map(|data| {
                            if let Err(_) = data {
                                return view! { <p>"Error loading the data"</p> }.into_any();
                            }
                            let portfolio = data.unwrap();
                            view! {
                                <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                    <Sidebar />
                                    <div class="lg:w-3/4 space-y-6 pb-12">
                                        {portfolio
                                            .sections()
                                            .into_iter()
                                            .map(|content| {
                                                match content {
                                                    PortfolioSection::AboutMe(section) => {
                                                        EitherOf5::A(
                                                            view! {
                                                                <AboutMeSection
                                                                    subtitle=section.subtitle().to_string()
                                                                    title=section.title().to_string()
                                                                    text=section.text().to_string()
                                                                    skills=section.skills().clone()
                                                                    profile_picture=section.profile_picture().clone()
                                                                    years_of_experience=section.years_of_experience().clone()
                                                                    cv_document=section.cv_document().clone()
                                                                />
                                                            },
                                                        )
                                                    }
                                                    PortfolioSection::Resume(section) => {
                                                        EitherOf5::B(
                                                            view! {
                                                                <ResumeSection
                                                                    title=section.title().to_string()
                                                                    subtitle=section.subtitle().to_string()
                                                                    text=section.text().to_string()
                                                                    education=section.education().clone()
                                                                    experience=section.experience().clone()
                                                                />
                                                            },
                                                        )
                                                    }
                                                    PortfolioSection::Projects(section) => {
                                                        EitherOf5::C(
                                                            view! {
                                                                <ProjectsSection
                                                                    title=section.title().to_string()
                                                                    subtitle=section.subtitle().to_string()
                                                                    text=section.text().to_string()
                                                                    projects=section.projects().clone()
                                                                />
                                                            },
                                                        )
                                                    },
                                                    PortfolioSection::Blogs(section) => {
                                                        EitherOf5::D(
                                                            view! {
                                                                <BlogSection
                                                                    title=section.title().to_string()
                                                                    subtitle=section.subtitle().to_string()
                                                                    text=section.text().to_string()
                                                                    articles=section.articles().clone()
                                                                />
                                                            },
                                                        )
                                                    },
                                                    _ => EitherOf5::E(view! { <MissingSection /> }),
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                                .into_any()
                        })
                }}
            </Suspense>
        </BasicLayout>
    }
}
