use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn WipeOverlay() -> impl IntoView {
    use leptos_router::hooks::use_navigate;

    let navigate = use_navigate();
    let pending: RwSignal<Option<String>> = RwSignal::new(None);
    // 0 = idle, 1 = wipe-in (covering screen), 2 = wipe-out (revealing new page)
    let phase = RwSignal::new(0u8);

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            let Some(window) = web_sys::window() else { return };
            let Some(doc) = window.document() else { return };

            let closure = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                if phase.get_untracked() != 0 { return; }
                let Some(target) = e.target() else { return };
                let Ok(el) = target.dyn_into::<web_sys::Element>() else { return };
                let Ok(Some(anchor)) = el.closest("a") else { return };
                let Some(href) = anchor.get_attribute("href") else { return };
                if href.starts_with('/') && !href.starts_with("//") {
                    e.prevent_default();
                    pending.set(Some(href));
                    phase.set(1);
                }
            });

            let _ = doc.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    let navigate = StoredValue::new(navigate);

    let wipe_class = move || match phase.get() {
        1 => "wipe in",
        2 => "wipe out",
        _ => "wipe",
    };

    view! {
        <Style id="comp-wipe">{include_str!("./wipe_overlay.css")}</Style>
        <div
            class=wipe_class
            on:animationend=move |_| {
                let p = phase.get_untracked();
                if p == 1 {
                    if let Some(url) = pending.get_untracked() {
                        navigate.with_value(|nav| nav(&url, Default::default()));
                        #[cfg(feature = "hydrate")]
                        if let Some(w) = web_sys::window() {
                            let _ = w.scroll_to_with_x_and_y(0.0, 0.0);
                        }
                        pending.set(None);
                    }
                    phase.set(2);
                } else if p == 2 {
                    phase.set(0);
                }
            }
        />
    }
}
