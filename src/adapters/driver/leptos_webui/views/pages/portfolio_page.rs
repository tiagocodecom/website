use leptos::prelude::*;
use leptos_meta::*;

use crate::adapters::driver::leptos_webui::controllers::portfolio_detail_controller;
use crate::adapters::driver::leptos_webui::views::components::common::UnexpectedError;
use crate::adapters::driver::leptos_webui::views::components::portfolio::*;
use crate::adapters::driver::leptos_webui::views::layouts::*;

#[component]
pub fn PortfolioPage() -> impl IntoView {
    let page_data = OnceResource::new(portfolio_detail_controller());

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div class="bg-main"></div> } }>
                {move || {
                    page_data
                    .get_untracked()
                    .map(|data| {
                        if let Err(_) = data {
                            return view! { <UnexpectedError /> }.into_any();
                        }

                        let portfolio = data.unwrap();
                        view! {
                            <Title text=portfolio.title().to_string() />
                            <div class="justify-center lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                <div class="hidden lg:block z-10 sticky top-2 lg:top-[80px] lg:h-fit lg:w-1/4 bg-black/90 dark:bg-boxDark backdrop-blur-[5px] px-4 py-3 lg:bg-black lg:px-8 lg:py-5 xl:px-10 xl:py-7 lg:backdrop-blur-none">
                                    <Sidebar />
                                </div>
                                <div class="lg:w-3/4 space-y-6">
                                    <DynamicSections sections=portfolio.sections().clone() />
                                </div>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}
