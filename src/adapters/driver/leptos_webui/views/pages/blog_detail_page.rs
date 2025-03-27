use leptos::prelude::*;
use leptos_meta::{Meta, Script, Title};
use leptos_router::hooks::use_location;

use crate::adapters::driver::leptos_webui::controllers::article_detail_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::{DynamicContent, Header};
use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::adapters::driver::leptos_webui::views::layouts::BasicLayout;

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let route = use_location();
    let page_data = Resource::new(
        move || route.pathname.read().to_string(),
        |slug| article_detail_controller(slug),
    );

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div>"Loading..."</div> } }>
                {move || {
                    page_data
                    .get_untracked()
                    .map(|data| {
                        if let Err(_) = data {
                            return view! { <UnexpectedError /> }.into_any();
                        }

                        let article = data.unwrap();
                        view! {
                            <Title text=article.title().to_string() />
                            <Meta name="description" content=article.summary().to_string() />
                            <Script src="/assets/plugins/highlightjs/highlight.min.js" />
                            <Script src="//cdnjs.cloudflare.com/ajax/libs/highlightjs-line-numbers.js/2.9.0/highlightjs-line-numbers.min.js" />

                            <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                <div class="lg:w-3/4 pb-12 article-detail section bg-white dark:bg-boxDark  px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
                                    <Header article=article.clone() />
                                    <DynamicContent content=article.content().clone() />
                                </div>
                            </div>

                            <script>
                                "document.addEventListener('DOMContentLoaded', (event) => {
                                    if (window.hljs) {
                                        window.hljs.highlightAll();
                                        window.hljs.initLineNumbersOnLoad();
                                    }
                                });"
                            </script>
                        }.into_any()
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}
