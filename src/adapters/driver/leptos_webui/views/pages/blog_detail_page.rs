use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::controllers::article_controller;
use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::adapters::driver::leptos_webui::views::layouts::BasicLayout;

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let page_data = OnceResource::new(article_controller());

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div>"Loading..."</div> } }>
                    {move || {
                        page_data
                        .get_untracked()
                        .map(|data| {
                            match data {
                                Err(_) => view! { <GeneralError /> }.into_any(),
                                Ok((categories, articles)) => view! {
                                    <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                        <div class="w-full space-y-6 mb-12">
                                            <DefaultContainer>
                                                <h1 class="text-4xl font-bold text-center">Blog Detail</h1>
                                            </DefaultContainer>
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
