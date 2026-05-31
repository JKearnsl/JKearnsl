use leptos::prelude::*;
use crate::presentation::components::ui::chip::{Chip, ChipRow};

struct GalCat {
    id: &'static str,
    label: &'static str,
    color: &'static str,
    hint: &'static str,
    count: usize,
}

const GAL_CATS: &[GalCat] = &[
    GalCat { id: "anime", label: "аниме",     color: "var(--terracotta)", hint: "кадр / арт", count: 8 },
    GalCat { id: "build", label: "постройки", color: "var(--ochre)",      hint: "архитектура", count: 8 },
];

#[component]
pub fn Page() -> impl IntoView {
    let cat = RwSignal::new("anime");

    view! {
        <main class="page">
            <section class="gallery-hero">
                <div class="wrap">
                    <div class="type-eyebrow gallery-eyebrow">
                        <span class="eyebrow-line"/>
                        "галерея"
                    </div>
                    <div class="gallery-header">
                        <h1 class="h-section">
                            "что радует "
                            <span class="italic-serif" style="color:var(--terracotta)">"глаз"</span>
                        </h1>
                        <p class="type-mono muted gallery-hint">
                            "// перетащи картинку на плитку — она сохранится в браузере."
                            <br/>
                            "// клик по плитке — выбрать файл."
                        </p>
                    </div>
                    <ChipRow>
                        {GAL_CATS.iter().map(|c| {
                            let id = c.id;
                            view! {
                                <Chip
                                    active=move || cat.get() == id
                                    on_click=move |_| cat.set(id)
                                >
                                    <span class="chip-dot" style=format!("background:{}", c.color)/>
                                    {c.label}
                                    <span class="muted">" "{c.count}</span>
                                </Chip>
                            }
                        }).collect_view()}
                    </ChipRow>
                </div>
            </section>

            <section class="gallery-grid-section">
                <div class="wrap">
                    <div class="gal-cols">
                        {move || {
                            let current = GAL_CATS.iter().find(|c| c.id == cat.get()).unwrap();
                            (0..current.count).map(|i| {
                                let slot_id = format!("{}-{}", cat.get(), i + 1);
                                let hint = current.hint;
                                let color = current.color;
                                let cap = format!("{:02}", i + 1);
                                let ar = match i % 3 {
                                    0 => "3/4",
                                    1 => "1/1",
                                    _ => "4/5",
                                };
                                view! {
                                    <ImageSlot id=slot_id ar=ar cap=cap color=color hint=hint/>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn ImageSlot(
    id: String,
    ar: &'static str,
    cap: String,
    color: &'static str,
    hint: &'static str,
) -> impl IntoView {
    let src = RwSignal::new(None::<String>);
    let over = RwSignal::new(false);
    let storage_key = format!("gal:{id}");

    // Load from localStorage on mount
    Effect::new({
        let _key = storage_key.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            {
                if let Some(win) = web_sys::window() {
                    if let Some(storage) = win.local_storage().ok().flatten() {
                        if let Ok(Some(data)) = storage.get_item(&_key) {
                            src.set(Some(data));
                        }
                    }
                }
            }
        }
    });

    let cap_clone = cap.clone();
    let _storage_key_for_clear = storage_key.clone();

    let clear_src = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        src.set(None);
        #[cfg(feature = "hydrate")]
        {
            if let Some(win) = web_sys::window() {
                if let Some(storage) = win.local_storage().ok().flatten() {
                    storage.remove_item(&_storage_key_for_clear).ok();
                }
            }
        }
    };

    view! {
        <div
            class="gal-slot card"
            style=format!("aspect-ratio:{}", ar)
            class:slot-over=move || over.get()
            on:dragover=move |ev| { ev.prevent_default(); over.set(true); }
            on:dragleave=move |_| over.set(false)
            on:drop=move |_ev| { over.set(false); }
        >
            {move || if let Some(data) = src.get() {
                view! {
                    <div class="slot-filled">
                        <img src=data alt="" class="slot-img"/>
                        <button class="slot-clear" on:click=clear_src.clone()>"×"</button>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="slot-empty">
                        <div class="slot-hatching"/>
                        <span class="slot-dot" style=format!("background:{}", color)/>
                        <div class="slot-footer">
                            <span class="type-mono muted">{hint}" · "{cap_clone.clone()}</span>
                            <span class="type-mono">"＋ drop"</span>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
