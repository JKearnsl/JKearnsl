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
            <section class="pt-[56px] pb-7">
                <div class="wrap">
                    <div class="type-eyebrow flex items-center gap-3">
                        <span class="inline-block w-[36px] h-px bg-current"/>
                        "галерея"
                    </div>
                    <div class="flex justify-between items-end gap-6 flex-wrap mt-[18px]">
                        <h1 class="h-section">
                            "что радует "
                            <span class="italic-serif" style="color:var(--terracotta)">"глаз"</span>
                        </h1>
                        <p class="type-mono muted max-w-[340px] leading-[1.7]">
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
                                    <span class="inline-block size-[6px] rounded-full" style=format!("background:{}", c.color)/>
                                    {c.label}
                                    <span class="muted">" "{c.count}</span>
                                </Chip>
                            }
                        }).collect_view()}
                    </ChipRow>
                </div>
            </section>

            <section class="pt-2 pb-[88px]">
                <div class="wrap">
                    <div class="columns-3 [column-gap:18px] max-[900px]:columns-2 max-[560px]:columns-1">
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
            class=move || format!("group relative w-full border {} bg-cream-2 overflow-hidden cursor-pointer mb-[18px] break-inside-avoid",
                if over.get() { "border-terracotta outline outline-2 outline-terracotta [outline-offset:-4px]" } else { "border-ink" })
            style=format!("aspect-ratio:{}", ar)
            on:dragover=move |ev| { ev.prevent_default(); over.set(true); }
            on:dragleave=move |_| over.set(false)
            on:drop=move |_ev| { over.set(false); }
        >
            {move || if let Some(data) = src.get() {
                view! {
                    <div class="absolute inset-0">
                        <img src=data alt="" class="absolute inset-0 w-full h-full object-cover block"/>
                        <button class="absolute top-[8px] right-[8px] size-[28px] rounded-full bg-[color-mix(in_oklab,var(--ink)_78%,transparent)] text-cream inline-flex items-center justify-center text-[14px] leading-none opacity-0 transition-opacity duration-[180ms] ease group-hover:opacity-100" on:click=clear_src.clone()>"×"</button>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="absolute inset-0">
                        <div class="absolute inset-0 [background-image:repeating-linear-gradient(45deg,var(--line)_0_1px,transparent_1px_11px)] opacity-90"/>
                        <span class="absolute left-3 top-3 size-[9px] rounded-full" style=format!("background:{}", color)/>
                        <div class="absolute left-0 right-0 bottom-0 py-3 px-[14px] flex justify-between items-end gap-2 font-mono text-[11px] tracking-[.04em] uppercase text-muted">
                            <span class="type-mono muted">{hint}" · "{cap_clone.clone()}</span>
                            <span class="type-mono">"＋ drop"</span>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
