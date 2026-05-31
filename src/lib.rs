#![recursion_limit = "512"]

pub mod presentation;
pub mod domain;

#[cfg(feature = "ssr")]
pub mod adapters;
#[cfg(feature = "ssr")]
pub mod ioc;
#[cfg(feature = "ssr")]
pub mod interactor_factory;
#[cfg(feature = "ssr")]
pub mod application;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use presentation::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
