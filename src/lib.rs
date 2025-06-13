pub mod presentation;
pub mod adapters;
pub mod ioc;
pub mod interactor_factory;
pub mod application;
pub mod domain;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use presentation::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
