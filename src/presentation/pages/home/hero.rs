use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::presentation::components::notes::{category, format, cover};
use crate::presentation::components::ui::badge::Badge;
use crate::presentation::components::ui::button::{Button, Variant as BtnVariant};
use crate::presentation::components::ui::chip::{Tag, ChipRow};
use crate::presentation::components::ui::card::Card;

#[component]
pub fn Editorial(notes: Resource<Vec<NoteListItem>>) -> impl IntoView {
    view! {
        <section class="hero-editorial">
            <Suspense fallback=move || view! { <div class="hero-skeleton"/> }>
                {move || notes.get().map(|posts| {
                    let post = posts.into_iter().next();
                    if let Some(post) = post {
                        view! { <Content post/> }.into_any()
                    } else {
                        view! { <div class="hero-empty type-mono">"// записей ещё нет"</div> }.into_any()
                    }
                })}
            </Suspense>
        </section>
    }.into_any()
}


#[component]
fn Content(post: NoteListItem) -> impl IntoView {
    let href = format!("/posts/{}", post.slug);
    let cat_label = category::label(&post.category);
    let cat_color = category::color(&post.category);
    let date_fmt = format::date(&post.created_at);
    let no_fmt = format!("{:03}", post.no);
    let tags = post.tags.clone();
    let title = post.title.clone();
    let description = post.description.clone();
    view! {
        <div class="wrap">
            <div class="hero-meta-bar">
                <span class="hero-label">"// последняя запись"</span>
                <span class="type-mono muted">{date_fmt}</span>
            </div>
            <div class="ed-grid">
                // Left: text
                <div class="ed-text">
                    <div class="ed-cat-row">
                        <Badge color=cat_color>{cat_label}</Badge>
                    </div>
                    <h1 class="h-display ed-title">
                        <a href={href.clone()}>{title}</a>
                    </h1>
                    <p class="ed-excerpt italic-serif">{description}</p>
                    <ChipRow>
                        {tags.into_iter().map(|t| view! {
                            <Tag>"#"{t}</Tag>
                        }).collect_view()}
                    </ChipRow>
                    <div class="ed-foot">
                        <div class="author-row">
                            <div class="author-avatar">"JK"</div>
                            <div>
                                <div class="type-mono-lg">"JKearnsl"</div>
                                <div class="type-mono muted">"автор · программист"</div>
                            </div>
                        </div>
                        <Button href=href.clone() variant=BtnVariant::Accent>"читать статью →"</Button>
                    </div>
                </div>
                // Right: cover
                <Card href=href class="ed-cover">
                    <cover::Art category=post.category.clone()/>
                    <div class="cover-badge">
                        <span class="cover-dot"/>
                        "№"{no_fmt}
                    </div>
                </Card>
            </div>
        </div>
    }
}
