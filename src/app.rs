use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::pages::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Tiagocode is a website about programming"/>
                <link rel="stylesheet" id="leptos" href="/pkg/tiagocode_website.css"/>
                <link rel="stylesheet" href="/assets/plugins/bootstrap-icons/bootstrap-icons.min.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body class="overflow-x-hidden bg-bodyBg font-opensans dark:bg-black">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Welcome to Tiagocode"/>
        <Router>
            <Routes fallback=NotFoundPage>
                <Route path=path!("") view=PortfolioPage/>
            </Routes>
        </Router>
    }
}
