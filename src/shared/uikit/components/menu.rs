use crate::shared::uikit::models::{Image, MenuItem};
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn Menu(
    items: Vec<MenuItem>,
    #[prop(default = "")] item_class: &'static str,
    #[prop(default = "")] container_class: &'static str,
) -> impl IntoView {
    view! {
        <ul class=container_class>
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <MenuItem
                            url=item.url().clone()
                            icon=item.icon().clone()
                            title=item.title().clone()
                            class=item_class.to_string()
                        />
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[component]
pub fn MenuItem(url: String, title: String, class: String, icon: Option<Image>) -> impl IntoView {
    view! {
        <li class=class
            .clone()>

            {if let Some(icon) = icon.clone() {
                Either::Left(
                    view! {
                        <a target="_blank" href=url.clone() title=title.clone() class=class.clone()>
                            <img src=icon.url().clone() alt=icon.alt().clone() class="h-8" />
                        </a>
                    },
                )
            } else {
                Either::Right(
                    view! {
                        <a target="_blank" href=url.clone() title=title.clone() class=class.clone()>
                            <span class="h-8">{title.clone()}</span>
                        </a>
                    },
                )
            }}
        </li>
    }
}
