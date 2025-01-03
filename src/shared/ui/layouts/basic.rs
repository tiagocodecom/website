use crate::shared::ui::components::{BackgroundAnimation, Navbar};
use leptos::prelude::*;

#[component]
pub fn BasicLayout(children: Children) -> impl IntoView {
    view! {
        <Navbar />
        <div class="container max-w-[1320px] mx-auto px-5 xl:px-3 pt-[64px]">{children()}</div>
        <BackgroundAnimation />
    }
}
