use leptos::prelude::*;
use crate::components::portfolio::*;

#[component]
pub fn PortfolioPage() -> impl IntoView {
    view! {
        <div class="container max-w-[1320px] mx-auto px-5 xl:px-3">
            <div class="lg:flex lg:justify-center items-center">
                <div class="py-7 lg:order-1">
                    <h1 class="text-5xl xl:text-7xl font-poppins font-semibold dark:text-white">Santiago <span class="stroke-text">Marulanda</span></h1>
                </div>
            </div>

            <div class="space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                <SidebarMenu />
                <div class="lg:w-3/4 space-y-6 pb-12">
                    <AboutMe />
                    <Resume />
                    <LatestWork />
                    <Blog />
                </div>
            </div>
        </div>
    }
}