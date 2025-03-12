use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, WildcardSegment,
};

use crate::adapters::driver::leptos_webui::views::pages::{
    BlogDetailPage, BlogPage, NotFoundPage, PortfolioPage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://local-admin.tiagocode.com" />
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/website.css"/>
        <Stylesheet href="/assets/css/custom.css"/>
        <Stylesheet href="/assets/plugins/bootstrap-icons/bootstrap-icons.min.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main class="bg-bodyBg">
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=PortfolioPage/>
                    <Route path=path!("/articles") view=BlogPage/>
                    <Route path=path!("/articles/:slug") view=BlogDetailPage/>
                    <Route path=WildcardSegment("any") view=NotFoundPage/>
                </Routes>
            </main>
        </Router>
    }
}
