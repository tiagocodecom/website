use crate::features::portfolio::components::*;
use crate::features::portfolio::models::{Portfolio, Section};
use crate::shared::ui::components::MissingSection;
use crate::shared::ui::layouts::BasicLayout;
use leptos::either::EitherOf5;
use leptos::prelude::*;

#[server]
async fn get_page_data() -> Result<Portfolio, ServerFnError> {
    use crate::features::portfolio::adapters::adapt_node_portfolio;
    use crate::shared::api::services::{ApiGetService, NodePortfolioService};
    use crate::shared::api::utils::HttpClient;
    use actix_web::web::Data;

    let http_client: Data<HttpClient> = leptos_actix::extract().await?;
    let node_portfolio = NodePortfolioService::new(http_client.into_inner())
        .fetch("/portfolio/santiago-marulanda")
        .await?;

    let portfolio = adapt_node_portfolio(node_portfolio).unwrap();

    Ok(portfolio)
}

#[component]
pub fn PortfolioPage() -> impl IntoView {
    let page_data = OnceResource::new(get_page_data());

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
                                <div class="lg:flex lg:justify-center items-center">
                                    <div class="py-7 lg:order-1">
                                        <h1 class="text-5xl xl:text-7xl font-poppins font-semibold dark:text-white text-center">
                                            {portfolio.title().clone()}
                                        </h1>
                                    </div>
                                </div>
                                <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                    <Sidebar />
                                    <div class="lg:w-3/4 space-y-6 pb-12">
                                        {portfolio
                                            .sections()
                                            .into_iter()
                                            .map(|content| {
                                                match content {
                                                    Section::AboutMe(about_me) => {
                                                        EitherOf5::A(
                                                            view! {
                                                                <AboutMeSection
                                                                    subtitle=about_me.subtitle().clone()
                                                                    title=about_me.title().clone()
                                                                    text=about_me.text().clone()
                                                                    skills=about_me.skills().clone()
                                                                    profile_picture=about_me.profile_picture().clone()
                                                                    years_of_experience=about_me.years_of_experience().clone()
                                                                    cv_document=about_me.cv_document().clone()
                                                                />
                                                            },
                                                        )
                                                    }
                                                    Section::Resume(resume) => {
                                                        EitherOf5::B(
                                                            view! {
                                                                <ResumeSection
                                                                    title=resume.title().clone()
                                                                    subtitle=resume.subtitle().clone()
                                                                    text=resume.text().clone()
                                                                    education=resume.education().clone()
                                                                    experience=resume.experience().clone()
                                                                />
                                                            },
                                                        )
                                                    }
                                                    Section::Projects(projects) => {
                                                        EitherOf5::C(
                                                            view! {
                                                                <ProjectsSection
                                                                    title=projects.title().clone()
                                                                    subtitle=projects.subtitle().clone()
                                                                    text=projects.text().clone()
                                                                    projects=projects.projects().clone()
                                                                />
                                                            },
                                                        )
                                                    }
                                                    Section::Blogs(_blogs) => EitherOf5::D(view! { <MissingSection /> }),
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
