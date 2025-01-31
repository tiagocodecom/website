use crate::shared::api::utils::HttpClient;
use crate::shared::uikit::components::{BackgroundAnimation, Navbar};
use crate::shared::uikit::models::Layout;
use leptos::either::Either;
use leptos::prelude::*;

#[server]
async fn get_layout_data() -> Result<Layout, ServerFnError> {
    use crate::shared::api::services::{ApiGetService, MenuItemsService};
    use crate::shared::uikit::adapters::layout_adapter;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let http_client: Data<HttpClient> = extract().await?;
    let menu_items_service = MenuItemsService::new(http_client.into_inner());

    let main_menu = menu_items_service.fetch("main").await?;
    let networks_menu = menu_items_service.fetch("social-network").await?;

    let layout = layout_adapter(networks_menu, main_menu);

    Ok(layout)
}

#[component]
pub fn BasicLayout(children: Children) -> impl IntoView {
    let layout_data = OnceResource::new(get_layout_data());

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
                                    logo=layout_data.logo().clone()
                                    main_menu=layout_data.main_menu().clone()
                                    social_menu=layout_data.social_menu().clone()
                                />
                            },
                        )
                    })
            }} <div class="container max-w-[1320px] mx-auto px-5 xl:px-3 pt-[64px]">{children()}</div>
            <BackgroundAnimation />
        </Transition>
    }
}
