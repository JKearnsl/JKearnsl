use leptos::prelude::*;
use leptos_meta::Style;
use crate::presentation::components::notes::{category, read_time, format, cover};
use crate::presentation::components::ui::badge::Badge;
use crate::presentation::components::ui::chip::{Chip, Tag, ChipRow};

#[component]
pub fn NoteContent(post: crate::domain::models::note::Note) -> impl IntoView {
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
        <Style id="notes-note-content">{include_str!("./note_content.css")}</Style>

        // progress bar
        <div
            class="progress-bar"
            style=move || format!("width:{}%", progress.get() * 100.0)
        />

        <article class="note-wrap">
            <div class="wrap note-inner">
                <a href="/" class="breadcrumb type-mono">"← / архив / "{cat_lbl}</a>

                <div class="note-meta-row">
                    <Badge color=cat_clr>{cat_lbl}</Badge>
                    <span class="type-mono muted">"№"{no_fmt}</span>
                    <span class="type-mono muted">{date}</span>
                    <span class="type-mono muted">{rt}</span>
                </div>

                <h1 class="h-display note-title">{title}</h1>
                <p class="note-lead">{description}</p>

                <div class="author-row" style="margin-top:36px">
                    <div class="author-avatar">"JK"</div>
                    <div>
                        <div class="type-mono-lg">"JKearnsl"</div>
                        <div class="type-mono muted">"автор · программист · вечный студент"</div>
                    </div>
                </div>
            </div>

            <div class="wrap note-cover-wrap">
                <div class="note-cover">
                    <cover::Art category=category_for_cover/>
                </div>
            </div>

            <div class="wrap note-body-wrap">
                <div class="note-body-grid">
                    <div class="prose" inner_html=body_html/>
                    <aside class="note-sidebar">
                        <div class="sidebar-block">
                            <div class="type-eyebrow">"// share"</div>
                            <ChipRow>
                                <Chip active=|| false on_click=|_| {}>"copy link"</Chip>
                                <Chip active=|| false on_click=|_| {}>"telegram"</Chip>
                                <Chip active=|| false on_click=|_| {}>"rss"</Chip>
                            </ChipRow>
                        </div>
                        <div class="sidebar-block">
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
