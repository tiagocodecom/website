use leptos::prelude::*;

#[component]
pub fn Container(
    children: Children,
    #[prop(default = "")]
    id: &'static str,
    #[prop(default = "section bg-white dark:bg-boxDark rounded-lg px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]")]
    class: &'static str
) -> impl IntoView {
    view! {
        <div id=id class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ContainerSubtitle(children: Children, #[prop(default = "")] backdrop_text: &'static str) -> impl IntoView {
    view! {
        <h6 data-backdrop-text=backdrop_text class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 dark:text-white before:content-['//'] before:pr-2 after:content-[attr(data-backdrop-text)] after:absolute after:top-0 after:left-0 after:font-poppins after:font-bold after:uppercase after:text-4xl after:opacity-15">
            {children()}
        </h6>
    }
}

#[component]
pub fn ContainerTitle(children: Children) -> impl IntoView {
    view! {
        <h2 class="text-4xl font-poppins font-semibold mb-2 dark:text-white">
            {children()}
        </h2>
    }
}

#[component]
pub fn ContainerDescription(children: Children) -> impl IntoView {
    view! {
        <p class="text-pColor dark:text-white/70">
            {children()}
        </p>
    }
}