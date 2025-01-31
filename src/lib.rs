pub mod app;
pub mod cms_content;
pub mod portfolio;
pub mod uikit;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::prelude::*;

    hydrate_body(App);
}
