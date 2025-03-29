use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::Logo;
use crate::adapters::driver::leptos_webui::views::components::common::Menu;
use crate::application::domain::layout::MenuTree;

#[component]
pub fn Navbar(main_menu: MenuTree, social_menu: MenuTree) -> impl IntoView {
    view! {
        <nav>
            <div class="flex flex-wrap items-center justify-between max-w-[1320px] mx-auto py-4 px-5 xl:px-0">
                <Logo />
                <div class="flex items-center">
                    <div class="md:order-2">
                        <Menu
                            items=social_menu.items().clone()
                            item_class="block py-0 px-3 -mr-3"
                            container_class="flex items-center font-medium"
                        />
                    </div>
                    <div class="md:order-1 hidden md:block">
                        <Menu
                            items=main_menu.items().clone()
                            item_class="block py-0 px-3 uppercase"
                            container_class="flex items-center font-medium mr-4"
                        />
                    </div>
                </div>
                <div class="block md:hidden">
                    <HamburgerButton />
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn HamburgerButton() -> impl IntoView {
    view! {
        <button
            type="button"
            data-collapse-toggle="navbar-sticky"
            class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200"
            aria-controls="navbar-sticky"
            aria-expanded="false"
        >
            <span class="sr-only">Open main menu</span>
            <svg
                class="w-5 h-5"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 17 14"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M1 1h15M1 7h15M1 13h15"
                />
            </svg>
        </button>
    }
}
