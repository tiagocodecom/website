use leptos::prelude::*;

#[component]
pub fn Pill(text: String, #[prop(optional)] emoji: String) -> impl IntoView {
    view! {
        <div class="inline-block px-4 py-2 me-2 rounded-full border border-black/20 border-dashed text-pColor hover:text-black transition ease-linear duration-100">
           <span>{emoji}</span>
            <div class="inline-block font-mono text-sm">
                {text}
            </div>
        </div>
    }
}
