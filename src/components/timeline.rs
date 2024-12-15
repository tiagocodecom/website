use leptos::prelude::*;

#[component]
pub fn Timeline(children: Children, icon: &'static str) -> impl IntoView {
    view! {
        <div class="relative pl-5 space-y-7 before:content-[''] before:absolute before:top-0 before:left-0 before:w-[1px] before:h-full before:border-l before:border-black/20 dark:before:border-white/20 before:border-dashed">
            <div class="text-3xl dark:text-white">
                <i class=icon></i>
            </div>
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineItem(children: Children) -> impl IntoView {
    view! {
        <div class="group">
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineDate(children: Children) -> impl IntoView {
    view! {
        <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineTitle(children: Children) -> impl IntoView {
    view! {
        <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">
            {children()}
        </h4>
    }
}

#[component]
pub fn TimelineSubtitle(children: Children) -> impl IntoView {
    view! {
        <span class="text-pColor dark:text-white/70">{children()}</span>
    }
}

#[component]
pub fn TimelineDesc(children: Children) -> impl IntoView {
    view! {
        <p class="text-pColor dark:text-white/70">{children()}</p>
    }
}