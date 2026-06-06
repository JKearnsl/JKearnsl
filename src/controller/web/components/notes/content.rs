use leptos::prelude::*;
use crate::controller::web::components::{
    notes::{category, read_time, format, cover},
    ui::{
        badge::Badge,
        chip::{Chip, Tag, ChipRow},
    },
};

#[component]
pub fn Content(post: crate::domain::models::note::Note) -> impl IntoView {
    let progress = RwSignal::new(0.0_f64);

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::window;
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                if let Some(win) = window() {
                    let doc = win.document().unwrap();
                    let el = doc.document_element().unwrap();
                    let scroll = el.scroll_top() as f64;
                    let total = (el.scroll_height() - el.client_height()) as f64;
                    if total > 0.0 {
                        progress.set(scroll / total);
                    }
                }
            }) as Box<dyn Fn()>);
            if let Some(win) = window() {
                win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()).ok();
            }
            closure.forget();
        }
    });

    let cat = post.category.clone();
    let cat_lbl = category::label(&cat);
    let cat_clr = category::color(&cat);
    let date = format::date(&post.created_at);
    let rt = read_time::calc_mins(&post.body);
    let no_fmt = format!("{:03}", post.no);
    let tags = post.tags.clone();
    let title = post.title.clone();
    let description = post.description.clone();
    let body_html = post.body.clone();
    let category_for_cover = post.category.clone();

    view! {
        // progress bar
        <div
            class="fixed left-0 top-0 h-[3px] bg-terracotta z-50 [transition:width_.08s_linear]"
            style=move || format!("width:{}%", progress.get() * 100.0)
        />

        <article class="pt-[56px]">
            <div class="wrap max-w-[1080px]!">
                <a href="/" class="text-muted mb-12 inline-flex items-center gap-2 type-mono">"← / архив / "{cat_lbl}</a>

                <div class="flex gap-5 flex-wrap items-center mb-7">
                    <Badge color=cat_clr>{cat_lbl}</Badge>
                    <span class="type-mono muted">"№"{no_fmt}</span>
                    <span class="type-mono muted">{date}</span>
                    <span class="type-mono muted">{rt}</span>
                </div>

                <h1 class="h-display max-w-[980px] text-balance">{title}</h1>
                <p class="text-[22px] text-ink-2 max-w-[780px] mt-6 leading-[1.45]">{description}</p>

                <div class="flex items-center gap-[13px]" style="margin-top:36px">
                    <div class="size-[42px] rounded-full bg-terracotta flex items-center justify-center text-cream font-mono font-bold text-[14px] shrink-0">"JK"</div>
                    <div>
                        <div class="type-mono-lg">"JKearnsl"</div>
                        <div class="type-mono muted">"автор · программист · вечный студент"</div>
                    </div>
                </div>
            </div>

            <div class="wrap max-w-[1280px]! mt-12!">
                <div class="relative aspect-[16/8] rounded-[28px] overflow-hidden border border-[var(--line)]">
                    <cover::Art category=category_for_cover/>
                </div>
            </div>

            <div class="wrap max-w-[1080px]! mt-16! pb-[80px]">
                <div class="grid grid-cols-[1fr_220px] gap-[64px] max-[900px]:grid-cols-1">
                    <div class="prose" inner_html=body_html/>
                    <aside class="sticky top-[120px] self-start flex flex-col gap-6 max-[900px]:static">
                        <div class="flex flex-col gap-[10px]">
                            <div class="type-eyebrow">"// share"</div>
                            <ChipRow>
                                <Chip active=|| false on_click=|_| {}>"copy link"</Chip>
                                <Chip active=|| false on_click=|_| {}>"telegram"</Chip>
                                <Chip active=|| false on_click=|_| {}>"rss"</Chip>
                            </ChipRow>
                        </div>
                        <div class="flex flex-col gap-[10px]">
                            <div class="type-eyebrow">"// теги"</div>
                            <ChipRow>
                                {tags.into_iter().map(|t| view! {
                                    <Tag>"#"{t}</Tag>
                                }).collect_view()}
                            </ChipRow>
                        </div>
                    </aside>
                </div>
            </div>
        </article>
    }
}
