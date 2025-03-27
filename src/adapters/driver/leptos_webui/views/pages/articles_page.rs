use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::adapters::driver::leptos_webui::controllers::articles_list_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::ListSection;
use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::adapters::driver::leptos_webui::views::layouts::BasicLayout;

#[component]
pub fn ArticlesPage() -> impl IntoView {
    let params = use_params_map();

    let page_data = Resource::new(
        move || params.read().get("category"),
        |category| articles_list_controller(category),
    );

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div class="bg-main"></div> } }>
                {move || {
                    page_data
                    .get_untracked()
                    .map(|data| {
                        match data {
                            Err(_) => view! { <UnexpectedError /> }.into_any(),
                            Ok((categories, articles)) => view! {
                                <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                    <div class="w-full space-y-6 mb-12">
                                        <ListSection articles=articles categories=categories />
                                    </div>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}
