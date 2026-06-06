use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::controller::web::components::notes::card::Card;

#[component]
pub fn Grid(posts: Vec<NoteListItem>) -> impl IntoView {
    view! {
        <section class="pt-2 pb-[56px]">
            <div class="wrap">
                <div class="grid grid-cols-3 gap-[28px_24px] max-[900px]:grid-cols-2 max-[560px]:grid-cols-1">
                    {if posts.is_empty() {
                        view! {
                            <div class="col-span-full p-12 text-center text-muted type-mono">"// в этой категории пока пусто"</div>
                        }.into_any()
                    } else {
                        posts.into_iter().map(|post| view! {
                            <Card post/>
                        }).collect_view().into_any()
                    }}
                </div>
            </div>
        </section>
    }
}
