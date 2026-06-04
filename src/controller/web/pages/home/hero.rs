use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::controller::web::components::notes::{category, format, cover};
use crate::controller::web::components::ui::badge::Badge;
use crate::controller::web::components::ui::button::{Button, Variant as BtnVariant};
use crate::controller::web::components::ui::chip::{Tag, ChipRow};
use crate::controller::web::components::ui::card::Card;

#[component]
pub fn Editorial(notes: Resource<Vec<NoteListItem>>) -> impl IntoView {
    view! {
        <section class="pt-[30px] pb-[48px] relative">
            <Suspense fallback=move || view! { <div class="h-[560px] bg-cream-2 rounded-[var(--radius)]"/> }>
                {move || notes.get().map(|posts| {
                    let post = posts.into_iter().next();
                    if let Some(post) = post {
                        view! { <Content post/> }.into_any()
                    } else {
                        view! { <div class="p-20 text-center type-mono">"// записей ещё нет"</div> }.into_any()
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
            <div class="border-t border-[var(--line)] pt-[14px] flex justify-between items-center font-mono text-[11px] tracking-[.18em] uppercase text-muted flex-wrap gap-3">
                <span class="text-ink">"// последняя запись"</span>
                <span class="type-mono muted">{date_fmt}</span>
            </div>
            <div class="mt-[26px] grid grid-cols-[1.12fr_.88fr] gap-[clamp(32px,4vw,60px)] items-stretch max-[920px]:grid-cols-1 max-[920px]:gap-6">
                // Left: text
                <div class="flex flex-col">
                    <div class="flex items-center gap-3 flex-wrap">
                        <Badge color=cat_color>{cat_label}</Badge>
                    </div>
                    <h1 class="h-display mt-4 text-balance cursor-pointer">
                        <a href={href.clone()}>{title}</a>
                    </h1>
                    <p class="font-serif italic font-normal text-[clamp(19px,1.7vw,25px)] leading-[1.4] text-ink-2 max-w-[560px] mt-4 text-pretty italic-serif">{description}</p>
                    <ChipRow class="mt-[18px]">
                        {tags.into_iter().map(|t| view! {
                            <Tag>"#"{t}</Tag>
                        }).collect_view()}
                    </ChipRow>
                    <div class="flex justify-between items-center gap-5 flex-wrap mt-auto border-t border-[var(--line)] pt-[22px]">
                        <div class="flex items-center gap-[13px]">
                            <div class="size-[42px] rounded-full bg-terracotta flex items-center justify-center text-cream font-mono font-bold text-[14px] shrink-0">"JK"</div>
                            <div>
                                <div class="type-mono-lg">"JKearnsl"</div>
                                <div class="type-mono muted">"автор · программист"</div>
                            </div>
                        </div>
                        <Button href=href.clone() variant=BtnVariant::Accent>"читать статью →"</Button>
                    </div>
                </div>
                // Right: cover
                <Card href=href class="relative w-full min-h-[380px] self-stretch border border-ink rounded-[var(--radius)] overflow-hidden block p-0 max-[920px]:min-h-0 max-[920px]:aspect-[16/10]">
                    <cover::Art category=post.category.clone()/>
                    <div class="absolute left-0 top-6 bg-ink text-cream py-[9px] pl-[22px] pr-[18px] font-mono text-[11px] tracking-[.18em] uppercase inline-flex items-center gap-[9px]">
                        <span class="inline-block size-[7px] rounded-full bg-ochre"/>
                        "№"{no_fmt}
                    </div>
                </Card>
            </div>
        </div>
    }
}
