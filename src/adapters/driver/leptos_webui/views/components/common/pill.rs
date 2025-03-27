use leptos::prelude::*;

#[component]
pub fn Pill(
    text: String,
    #[prop(optional)] emoji: String,
    #[prop(optional, default = "".to_string())] link: String,
) -> impl IntoView {
    view! {
        <div class="inline-block px-4 py-2 me-2 rounded-full border border-black/20 border-dashed text-mainfont hover:text-black transition ease-linear duration-100">
            { if !link.is_empty() {
                view! {
                    <a href=link target="_self">
                        <span class=("mr-2", move || !emoji.is_empty())>{emoji.clone()}</span>
                        <span class="inline-block font-mono text-sm">
                            {text}
                        </span>
                    </a>
                }.into_any()
            } else {
                view! {
                    <span class=("mr-2", move || !emoji.is_empty())>{emoji.clone()}</span>
                    <span class="inline-block font-mono text-sm">
                        {text}
                    </span>
                }.into_any()
            }}
        </div>
    }
}
