use leptos::prelude::*;

#[component]
pub fn Badge(
    children: Children,
    #[prop(default = "bi bi-code-slash pe-1")] icon: &'static str,
) -> impl IntoView {
    view! {
        <div class="inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100">
            <i class=icon></i>
            <div class="inline-block font-mono text-sm">
                {children()}
            </div>
        </div>
    }
}
