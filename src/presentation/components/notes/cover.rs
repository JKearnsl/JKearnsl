use leptos::prelude::*;
use crate::domain::models::note::Category;

#[component]
pub fn Art(category: Category) -> impl IntoView {
    view! {
        {match category {
            Category::Math => view! { <Math/> }.into_any(),
            Category::Science => view! { <Science/> }.into_any(),
            Category::Prog => view! { <Prog/> }.into_any(),
        }}
    }
}

#[component]
fn Prog() -> impl IntoView {
    view! {
        <div class="absolute inset-0 flex items-center justify-center overflow-hidden" style="background:var(--terracotta)">
            <svg viewBox="0 0 200 200" width="78%" height="78%" class="cover-svg">
                {(0..7).map(|i| {
                    let x = 20 + i * 10;
                    let y = 20 + i * 10;
                    let w = 160 - i * 20;
                    let h = 160 - i * 20;
                    let rot = i * 4;
                    let opacity = format!("{:.2}", 0.9_f64 - i as f64 * 0.08);
                    view! {
                        <rect
                            x=x y=y width=w height=h
                            fill="none" stroke="var(--cream)" stroke-width="1.6"
                            transform=format!("rotate({} 100 100)", rot)
                            opacity=opacity
                        />
                    }
                }).collect_view()}
                <circle cx="100" cy="100" r="14" fill="var(--ochre)"/>
            </svg>
            <span class="cover-meta">"// прог"</span>
        </div>
    }
}

#[component]
fn Math() -> impl IntoView {
    view! {
        <div class="absolute inset-0 flex items-center justify-center overflow-hidden" style="background:var(--ochre)">
            <div class="cover-grid-bg"/>
            <div class="font-serif italic text-[clamp(80px,16vw,180px)] text-ink opacity-90 relative z-[1] leading-none italic-serif">"∑"</div>
            <span class="cover-meta">"// мат"</span>
        </div>
    }
}

#[component]
fn Science() -> impl IntoView {
    view! {
        <div class="absolute inset-0 flex items-center justify-center overflow-hidden" style="background:var(--plum)">
            <svg viewBox="0 0 200 200" width="100%" height="100%" preserveAspectRatio="none" class="absolute inset-0">
                <defs>
                    <radialGradient id="gs" cx=".5" cy=".5" r=".5">
                        <stop offset="0%" stop-color="var(--ochre)" stop-opacity=".85"/>
                        <stop offset="60%" stop-color="var(--terracotta)" stop-opacity=".4"/>
                        <stop offset="100%" stop-color="var(--plum)" stop-opacity="0"/>
                    </radialGradient>
                </defs>
                <circle cx="100" cy="100" r="60" fill="url(#gs)"/>
                {(0i32..3).map(|i| {
                    let rx = 60 + i * 18;
                    let ry = 20 + i * 6;
                    let rot = i * 55;
                    view! {
                        <ellipse cx="100" cy="100" rx=rx ry=ry fill="none"
                            stroke="var(--cream)" stroke-width=".7"
                            transform=format!("rotate({} 100 100)", rot) opacity=".75"/>
                    }
                }).collect_view()}
                <circle cx="36" cy="80" r="2.4" fill="var(--cream)"/>
                <circle cx="165" cy="120" r="1.8" fill="var(--cream)"/>
            </svg>
            <span class="cover-meta" style="color:var(--cream)">"// наука"</span>
        </div>
    }
}
