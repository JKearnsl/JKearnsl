use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::presentation::components::notes::{category, format, cover};

#[component]
pub fn Section(notes: Resource<Vec<NoteListItem>>) -> impl IntoView {
    view! {
        <section class="pt-[80px] pb-6">
            <div class="wrap">
                <div class="flex justify-between items-end gap-5 flex-wrap">
                    <div>
                        <div class="type-eyebrow">"// featured"</div>
                        <h2 class="h-section mt-2">"свежее"</h2>
                    </div>
                </div>
                <Suspense fallback=move || view! {<div/>}>
                    {move || notes.get().map(|posts| {
                        let featured: Vec<_> = posts.into_iter().skip(1).take(3).collect();
                        view! {
                            <div class="grid grid-cols-[1.5fr_1fr_1fr] gap-6 mt-9 max-[900px]:grid-cols-2 max-[600px]:grid-cols-1">
                                {featured.into_iter().enumerate().map(|(i, post)| view! {
                                    <FeaturedCard post big=i==0/>
                                }).collect_view()}
                            </div>
                        }
                    })}
                </Suspense>
            </div>
        </section>
    }.into_any()
}

#[component]
fn FeaturedCard(post: NoteListItem, big: bool) -> impl IntoView {
    let href = format!("/posts/{}", post.slug);
    let cat_label = category::label(&post.category);
    let cat_color = category::color(&post.category);
    let date_fmt = format::date(&post.created_at);
    let no_fmt = format!("{:03}", post.no);
    let title = post.title.clone();
    let description = post.description.clone();
    let category = post.category.clone();

    view! {
        <article class="card bg-paper border border-[var(--line)] rounded-[var(--radius)] overflow-hidden flex flex-col">
            <a href={href} class="no-underline overflow-hidden flex flex-col flex-1">
                <div class=move || if big { "relative aspect-[16/11] w-full" } else { "relative aspect-[5/4] w-full" }>
                    <cover::Art category/>
                </div>
                <div class="p-[22px_24px_26px] flex flex-col gap-[14px] flex-1">
                    <div class="flex items-center justify-between">
                        <span class="type-mono" style=format!("color:{}", cat_color)>{cat_label}</span>
                        <span class="type-mono muted">{date_fmt}</span>
                    </div>
                    <h3 class="h-card">{title.clone()}</h3>
                    {big.then(|| view! {
                        <p class="text-muted text-[15px] leading-[1.5]">{description}</p>
                    })}
                    <div class="mt-auto flex justify-between items-center pt-2">
                        <span class="type-mono muted">"№"{no_fmt}</span>
                        <span class="arrow-circle">"→"</span>
                    </div>
                </div>
            </a>
        </article>
    }
}
