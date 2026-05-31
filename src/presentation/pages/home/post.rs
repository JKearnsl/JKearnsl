use leptos::prelude::*;
use leptos_meta::Style;
use crate::domain::models::note::NoteListItem;
use crate::presentation::components::notes::note_card::NoteCard;

#[component]
pub fn Grid(posts: Vec<NoteListItem>) -> impl IntoView {
    view! {
        <Style id="home-post-grid">{include_str!("./post.css")}</Style>
        <section class="posts-section">
            <div class="wrap">
                <div class="posts-grid">
                    {if posts.is_empty() {
                        view! {
                            <div class="posts-empty type-mono">"// в этой категории пока пусто"</div>
                        }.into_any()
                    } else {
                        posts.into_iter().map(|post| view! {
                            <NoteCard post/>
                        }).collect_view().into_any()
                    }}
                </div>
            </div>
        </section>
    }
}
