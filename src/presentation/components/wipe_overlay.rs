use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn WipeOverlay() -> impl IntoView {
    use leptos_router::hooks::use_navigate;

    let navigate = use_navigate();
    #[allow(unused)]
    let navigate = StoredValue::new(navigate);
    // 0 = idle, 1 = wipe-in, 2 = wipe-out
    let phase = RwSignal::new(0u8);

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            let Some(window) = web_sys::window() else { return };
            let Some(doc) = window.document() else { return };

            let closure = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target() else { return };
                let Ok(el) = target.dyn_into::<web_sys::Element>() else { return };
                let Ok(Some(anchor)) = el.closest("a") else { return };
                let Some(href) = anchor.get_attribute("href") else { return };

                if href.starts_with('/') && !href.starts_with("//") {
                    e.prevent_default();

                    // Navigate immediately — don't wait for animation
                    navigate.with_value(|nav| nav(&href, Default::default()));

                    // Play the overlay animation only if idle
                    if phase.get_untracked() == 0 {
                        phase.set(1);
                    }
                }
            });

            let _ = doc.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    let wipe_class = move || match phase.get() {
        1 => "wipe in",
        2 => "wipe out",
        _ => "wipe",
    };

    view! {
        <Style id="comp-wipe">{include_str!("./wipe_overlay.css")}</Style>
        <div
            class=wipe_class
            on:animationend=move |_ev| {
                #[cfg(feature = "hydrate")]
                {
                    use wasm_bindgen::JsCast;
                    // Only react to animations on this element itself
                    if let Ok(anim_ev) = _ev.dyn_into::<web_sys::AnimationEvent>() {
                        let name = anim_ev.animation_name();
                        let p = phase.get_untracked();
                        if p == 1 && name == "wipeIn" {
                            phase.set(2);
                        } else if p == 2 && name == "wipeOut" {
                            phase.set(0);
                        }
                    }
                }
            }
        />
    }
}
