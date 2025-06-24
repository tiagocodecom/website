use leptos::either::EitherOf3;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::article::ArticleContent;

#[component]
pub fn DynamicContent(content: Vec<ArticleContent>) -> impl IntoView {
    view! {
        {content
            .into_iter()
            .map(|content| {
                match content {
                    ArticleContent::Image(img) => {
                        EitherOf3::A(view! { <Img image=img.clone() class="mt-6" /> })
                    },
                    ArticleContent::Text(text) => {
                        EitherOf3::B(view! { <RawHtml html=text.to_string() class="mt-6"/> })
                    },
                    _ => EitherOf3::C(view! {<MissingSection />})
                }
            }).collect_view()
        }
    }
}
