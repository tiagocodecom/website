use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{path, SsrMode, WildcardSegment};

use crate::adapters::driver::leptos_webui::views::pages::{
    ArticlesPage, BlogDetailPage, NotFoundPage, PortfolioPage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Router>
            <Stylesheet id="leptos" href="/pkg/website.css"/>
            <Stylesheet href="/assets/css/custom.css"/>
            <Stylesheet href="/assets/plugins/highlightjs/default.min.css"/>
            <Stylesheet href="/assets/plugins/bootstrap-icons/bootstrap-icons.min.css"/>
            <Title formatter=|text| format!("{text} - Tiagocode")/>
            <Routes fallback=move || "Not found.">
                <Route ssr=SsrMode::Async path=path!("/") view=PortfolioPage/>
                <Route ssr=SsrMode::Async path=path!("/articles") view=ArticlesPage/>
                <Route ssr=SsrMode::Async path=path!("/articles/:slug") view=BlogDetailPage/>
                <Route ssr=SsrMode::Async path=WildcardSegment("any") view=NotFoundPage/>
            </Routes>
        </Router>
    }
}
