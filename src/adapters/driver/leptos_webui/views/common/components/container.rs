use leptos::prelude::*;

#[component]
pub fn DefaultContainer(children: Children, #[prop(optional)] id: String) -> impl IntoView {
    view! {
        <div id=id class="section bg-white px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
            {children()}
        </div>
    }
}

#[component]
pub fn CustomContainer(children: Children, class: String) -> impl IntoView {
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ContainerH1Title(text: String) -> impl IntoView {
    view! { <h1 class="text-4xl font-poppins font-semibold mb-2">{text}</h1> }
}

#[component]
pub fn ContainerH2Title(text: String) -> impl IntoView {
    view! { <h2 class="text-4xl font-poppins font-semibold mb-2">{text}</h2> }
}

#[component]
pub fn ContainerSubtitle(text: String) -> impl IntoView {
    view! {
        <span class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 before:content-['//'] before:pr-2">
            {text}
        </span>
    }
}

#[component]
pub fn ContainerDescription(text: String) -> impl IntoView {
    view! { <p class="text-pColor">{text}</p> }
}
