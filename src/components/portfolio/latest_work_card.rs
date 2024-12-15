use leptos::prelude::*;

#[component]
pub fn LatestWorkCard(children: Children) -> impl IntoView {
    view! {
        <div class="portfolio-item category-1">
            <div class="relative overflow-hidden group rounded-lg after:content-[''] after:absolute after:top-0 after:left-0 after:w-full after:h-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:ease-out after:duration-[160ms] hover:after:opacity-100 category-1">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn LatestWorkCardLink(
    children: Children,
    #[prop(default = "")] href: &'static str
) -> impl IntoView {
    view! {
        <div class="z-[1] absolute bottom-0 left-0 w-full px-7 pb-6 invisible opacity-0 translate-y-2 group-hover:translate-y-0 group-hover:visible group-hover:opacity-100 group-hover:mb-0 transition ease-out duration-[160ms]">
            <a href=href class="font-poppins font-semibold text-3xl lg:text-4xl tracking-[0.5px] portfolio-stroke-text transition-all ease-linear duration-100">
                {children()}
            </a>
        </div>
    }
}

#[component]
pub fn LatestWorkCardImage(
    #[prop(default = "/assets/images/portfolio-img.png")] src: &'static str,
    #[prop(default = "")] alt: &'static str
) -> impl IntoView {
    view! {
        <img src=src alt=alt class="transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.4px]" />
    }
}