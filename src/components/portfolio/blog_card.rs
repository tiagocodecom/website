use leptos::prelude::*;

#[component]
pub fn BlogCard(children: Children) -> impl IntoView {
    view! {
        <div class="md:flex md:items-start">
            {children()}
        </div>
    }
}

#[component]
pub fn BlogCardImage(
    #[prop(default = "assets/images/blog-img.png")] src: &'static str,
    #[prop(default = "")] alt: &'static str,
    #[prop(default = "")] overlay_text: &'static str
) -> impl IntoView {
    view! {
        <div class="overflow-hidden relative rounded-lg group">
            <img src=src alt=alt class="md:max-w-[340px] transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]"/>
            <div class="absolute top-4 left-4 bg-black/20 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                {overlay_text}
            </div>
        </div>
    }
}

#[component]
pub fn BlogCardContent(children: Children) -> impl IntoView {
    view! {
        <div class="mt-5 md:pl-7 md:mt-0">
            {children()}
        </div>
    }
}

#[component]
pub fn BlogCardDate(children: Children) -> impl IntoView {
    view! {
        <span class="text-pColor dark:text-white/70">{children()}</span>
    }
}

#[component]
pub fn BlogCardTitle(children: Children) -> impl IntoView {
    view! {
        <h3 class="font-poppins font-semibold text-2xl lg:text-3xl mt-2">
            {children()}
        </h3>
    }
}

#[component]
pub fn BlogCardDescription(children: Children) -> impl IntoView {
    view! {
        <p class="text-pColor dark:text-white/70">
            {children()}
        </p>
    }
}

#[component]
pub fn BlogCardLink(#[prop(default = "#")] href: &'static str) -> impl IntoView {
    view! {
        <a href=href class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black">
            Read More
        </a>
    }
}