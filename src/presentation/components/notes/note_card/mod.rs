use leptos::prelude::*;
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
        <article class="relative [transition:transform_.35s_cubic-bezier(.2,.7,.2,1)] cursor-pointer hover:-translate-y-[6px]">
            <a href={href} class="overflow-hidden flex flex-col no-underline bg-paper border border-[var(--line)] rounded-[var(--radius)]">
                <div class="relative aspect-[5/4]">
                    <cover::Art category/>
                </div>
                <div class="p-[20px_22px_22px]">
                    <div class="flex items-center justify-between mb-3">
                        <span class="type-mono" style=format!("color:{}", cat_color)>{cat_label}</span>
                        <span class="type-mono muted">"№"{no_fmt}</span>
                    </div>
                    <h3 class="h-card text-[22px] text-pretty">{title}</h3>
                    <div class="flex items-center justify-between mt-[18px]">
                        <span class="type-mono muted">{date_fmt}</span>
                    </div>
                </div>
            </a>
        </article>
    }
}
