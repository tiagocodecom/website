use leptos::either::Either;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::controllers::get_layout_controller;
use crate::adapters::driver::leptos_webui::views::common::components::BackgroundAnimation;
use crate::adapters::driver::leptos_webui::views::common::components::Navbar;

#[component]
pub fn BasicLayout(children: Children) -> impl IntoView {
    let layout_data = OnceResource::new(get_layout_controller());

    view! {
        <Transition fallback=move || {
            view! { <div>"Loading..."</div> }
        }>
            {move || {
                layout_data
                    .get_untracked()
                    .map(|data| {
                        if let Err(e) = data {
                            return Either::Left(view! { <p>"Error loading the data: " {e.to_string()}</p> });
                        }
                        let layout_data = data.unwrap();
                        Either::Right(
                            view! {
                                <Navbar
                                    main_menu=layout_data.main_menu().clone()
                                    social_menu=layout_data.social_menu().clone()
                                />
                            },
                        )
                    })
            }}
                <div class="container max-w-[1320px] mx-auto px-5 xl:px-0 pt-[64px] lg:pt-[128px]">
                    {children()}
                </div>
                <footer class="text-center py-8 text-sm text-gray-500 dark:text-gray-400">
                    <p>"Made with love by Santiago Marulanda ❤️."</p>
                </footer>
            <BackgroundAnimation />
        </Transition>
    }
}
