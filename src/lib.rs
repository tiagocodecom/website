pub mod app;
pub mod pages;
pub mod components;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::prelude::*;

    hydrate_body(App);
}
