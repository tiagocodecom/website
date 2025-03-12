use leptos::prelude::*;

use crate::application::domain::common::Image;

#[component]
pub fn Img(image: Image, class: &'static str) -> impl IntoView {
    view! {
        <figure>
            <img class=class src=image.url().to_string() alt=image.alt().to_string() />
            <figcaption class="hidden">{image.alt().to_string()}</figcaption>
        </figure>
    }
}
