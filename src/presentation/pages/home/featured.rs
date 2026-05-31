use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::presentation::components::notes::{category, format, cover};

#[component]
pub fn Section(notes: Resource<Vec<NoteListItem>>) -> impl IntoView {
    view! {
        <section class="featured-section">
            <div class="wrap">
                <div class="section-header">
                    <div>
                        <div class="type-eyebrow">"// featured"</div>
                        <h2 class="h-section">"свежее"</h2>
                    </div>
                </div>
                <Suspense fallback=move || view! {<div/>}>
                    {move || notes.get().map(|posts| {
                        let featured: Vec<_> = posts.into_iter().skip(1).take(3).collect();
                        view! {
                            <div class="featured-grid">
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
        <article class="card card-surface featured-card">
            <a href={href} class="featured-card-inner">
                <div class="featured-cover" class:featured-cover-big=big>
                    <cover::Art category/>
                </div>
                <div class="featured-body">
                    <div class="featured-meta">
                        <span class="type-mono" style=format!("color:{}", cat_color)>{cat_label}</span>
                        <span class="type-mono muted">{date_fmt}</span>
                    </div>
                    <h3 class="h-card">{title.clone()}</h3>
                    {big.then(|| view! {
                        <p class="featured-excerpt muted">{description}</p>
                    })}
                    <div class="featured-footer">
                        <span class="type-mono muted">"№"{no_fmt}</span>
                        <span class="arrow-circle">"→"</span>
                    </div>
                </div>
            </a>
        </article>
    }
}
