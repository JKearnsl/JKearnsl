use leptos::prelude::*;
use leptos_meta::Style;
use crate::domain::models::note::NoteListItem;
use crate::presentation::components::notes::{category, format, cover};

#[component]
pub fn NoteCard(post: NoteListItem) -> impl IntoView {
    let href = format!("/posts/{}", post.slug);
    let cat_label = category::label(&post.category);
    let cat_color = category::color(&post.category);
    let date_fmt = format::date(&post.created_at);
    let no_fmt = format!("{:03}", post.no);
    let title = post.title.clone();
    let category = post.category.clone();

    view! {
        <Style id="ui-card">{include_str!("../../ui/card/card.css")}</Style>
        <Style id="notes-note-card">{include_str!("./note_card.css")}</Style>
        <article class="card note-card">
            <a href={href} class="note-card-inner card-surface">
                <div class="note-card-cover">
                    <cover::Art category/>
                </div>
                <div class="note-card-body">
                    <div class="note-card-meta">
                        <span class="type-mono" style=format!("color:{}", cat_color)>{cat_label}</span>
                        <span class="type-mono muted">"№"{no_fmt}</span>
                    </div>
                    <h3 class="h-card note-card-title">{title}</h3>
                    <div class="note-card-footer">
                        <span class="type-mono muted">{date_fmt}</span>
                    </div>
                </div>
            </a>
        </article>
    }
}
