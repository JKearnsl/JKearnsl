use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn WipeOverlay() -> impl IntoView {
    use leptos_router::hooks::use_navigate;

    let navigate = use_navigate();
    #[allow(unused)]
    let navigate = StoredValue::new(navigate);
    // 0 = idle, 1 = wipe-in (covering), 2 = wipe-out (revealing)
    let phase = RwSignal::new(0u8);
    // href waiting to be navigated to after wipe-in completes
    #[allow(unused)]
    let pending_href: StoredValue<Option<String>> = StoredValue::new(None);

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
                    if phase.get_untracked() != 0 { return; }
                    e.prevent_default();

                    pending_href.set_value(Some(href.clone()));
                    phase.set(1); // start wipe-in

                    // Fallback: if animationend doesn't fire within 600ms, navigate anyway.
                    // Animation is 450ms so 600ms gives comfortable headroom.
                    #[cfg(feature = "hydrate")]
                    if let Some(win) = web_sys::window() {
                        let href_fb = href.clone();
                        let cb = Closure::once(move || {
                            // Only act if animationend didn't already clear pending_href
                            if pending_href.get_value().is_some() {
                                pending_href.set_value(None);
                                navigate.with_value(|nav| nav(&href_fb, Default::default()));
                                phase.set(2);
                                // Fallback for wipe-out as well
                                if let Some(win2) = web_sys::window() {
                                    let cb2 = Closure::once(move || {
                                        if phase.get_untracked() == 2 {
                                            phase.set(0);
                                        }
                                    });
                                    let _ = win2.set_timeout_with_callback_and_timeout_and_arguments_0(
                                        cb2.as_ref().unchecked_ref(), 600,
                                    );
                                    cb2.forget();
                                }
                            }
                        });
                        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(), 600,
                        );
                        cb.forget();
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
                    if let Ok(anim_ev) = _ev.dyn_into::<web_sys::AnimationEvent>() {
                        let name = anim_ev.animation_name();
                        let p = phase.get_untracked();
                        if p == 1 && name == "wipeIn" {
                            // Screen is now fully covered — navigate, then start wipe-out
                            if let Some(href) = pending_href.get_value() {
                                pending_href.set_value(None);
                                navigate.with_value(|nav| nav(&href, Default::default()));
                            }
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
