use leptos::prelude::*;
use super::{TRACKS, fmt_time, track};

#[component]
pub fn Section(idx: RwSignal<usize>, playing: RwSignal<bool>, cur: RwSignal<u32>) -> AnyView {
    view! {
        <section class="pt-7 pb-6">
            <div class="wrap">
                <div class="grid grid-cols-[minmax(0,1fr)_minmax(0,1.1fr)] gap-[clamp(28px,3.5vw,56px)] items-stretch border border-ink bg-cream-2 overflow-hidden max-[760px]:grid-cols-1">
                    <div class="relative min-h-[360px] border-r border-ink max-[760px]:border-r-0 max-[760px]:border-b max-[760px]:border-ink max-[760px]:min-h-[240px]">
                        {move || {
                            let t = &TRACKS[idx.get()];
                            view! { <track::Art color=t.color kind=t.kind playing=playing.get()/> }.into_any()
                        }}
                    </div>
                    <Controls idx playing cur/>
                </div>
            </div>
        </section>
    }.into_any()
}

#[component]
fn Controls(idx: RwSignal<usize>, playing: RwSignal<bool>, cur: RwSignal<u32>) -> AnyView {
    let go_prev = move |_: leptos::ev::MouseEvent| idx.update(|i| *i = (*i + crate::presentation::pages::music::TRACKS.len() - 1) % crate::presentation::pages::music::TRACKS.len());
    let go_next = move |_: leptos::ev::MouseEvent| idx.update(|i| *i = (*i + 1) % crate::presentation::pages::music::TRACKS.len());
    let toggle  = move |_: leptos::ev::MouseEvent| playing.update(|p| *p = !*p);
    let track_dur = move || crate::presentation::pages::music::TRACKS[idx.get()].dur;
    let progress = Signal::derive(move || if track_dur() > 0 { cur.get() as f64 / track_dur() as f64 } else { 0.0 });

    view! {
        <div class="flex flex-col justify-center p-[clamp(28px,3vw,44px)]">
            <div class="type-mono muted">
                "now playing · №"{move || TRACKS[idx.get()].no}
            </div>
            <h2 class="font-display font-semibold text-[clamp(32px,3.4vw,52px)] tracking-[-0.03em] leading-[1.02] mt-3 mb-[6px] text-balance">{move || TRACKS[idx.get()].title}</h2>
            <div class="flex items-center gap-[10px] font-mono text-[13px] text-muted">
                <span class="type-mono">"JKearnsl"</span>
                <span class="type-mono muted">"·"</span>
                <span class="type-mono" style=move || format!("color:{}", TRACKS[idx.get()].color)>
                    {move || TRACKS[idx.get()].kind}
                </span>
            </div>

            <Scrubber idx cur progress/>

            <div class="flex items-center gap-4 mt-[22px]">
                <button class="size-11 rounded-full bg-transparent text-ink border border-[var(--line)] inline-flex items-center justify-center text-[14px] cursor-pointer" on:click=go_prev>"⏮"</button>
                <button class="size-16 rounded-full text-cream inline-flex items-center justify-center text-[22px] border border-ink transition-transform duration-150 hover:-translate-y-[2px]" on:click=toggle
                    style=move || format!("background:{}", TRACKS[idx.get()].color)>
                    {move || if playing.get() { "❚❚" } else { "▶" }}
                </button>
                <button class="size-11 rounded-full bg-transparent text-ink border border-[var(--line)] inline-flex items-center justify-center text-[14px] cursor-pointer" on:click=go_next>"⏭"</button>
                <span style="flex:1"/>
                <span class="type-mono muted">{move || idx.get() + 1}"/" {TRACKS.len()}</span>
            </div>

            <div class="mt-[22px] pt-4 border-t border-[var(--line)] text-[11.5px] leading-[1.6] type-mono muted">
                "// демо-режим. загрузи .mp3-файл для воспроизведения."
            </div>
        </div>
    }.into_any()
}

#[component]
fn Scrubber(idx: RwSignal<usize>, cur: RwSignal<u32>, #[prop(into)] progress: Signal<f64>) -> AnyView {
    view! {
        <div class="mt-7">
            <div class="relative h-[22px] flex items-center cursor-pointer">
                <div class="absolute left-0 right-0 h-[4px] bg-[var(--line)] rounded"/>
                <div class="absolute left-0 h-[4px] bg-terracotta rounded" style=move || format!("width:{}%", progress.get() * 100.0)/>
                <div class="absolute size-[14px] -ml-[7px] rounded-full bg-ink border-2 border-cream" style=move || format!("left:{}%", progress.get() * 100.0)/>
            </div>
            <div class="flex justify-between mt-2 font-mono text-[12px] text-muted">
                <span class="type-mono muted">{move || fmt_time(cur.get())}</span>
                <span class="type-mono muted">{move || fmt_time(TRACKS[idx.get()].dur)}</span>
            </div>
        </div>
    }.into_any()
}
