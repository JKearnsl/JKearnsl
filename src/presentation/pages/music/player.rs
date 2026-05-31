use leptos::prelude::*;
use super::{TRACKS, fmt_time, track};

#[component]
pub fn Section(idx: RwSignal<usize>, playing: RwSignal<bool>, cur: RwSignal<u32>) -> AnyView {
    view! {
        <section class="music-player-section">
            <div class="wrap">
                <div class="now-playing-grid">
                    <div class="np-art">
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
        <div class="np-controls">
            <div class="type-mono muted">
                "now playing · №"{move || TRACKS[idx.get()].no}
            </div>
            <h2 class="np-title">{move || TRACKS[idx.get()].title}</h2>
            <div class="np-meta">
                <span class="type-mono">"JKearnsl"</span>
                <span class="type-mono muted">"·"</span>
                <span class="type-mono" style=move || format!("color:{}", TRACKS[idx.get()].color)>
                    {move || TRACKS[idx.get()].kind}
                </span>
            </div>

            <Scrubber idx cur progress/>

            <div class="transport">
                <button class="transport-btn" on:click=go_prev>"⏮"</button>
                <button class="play-btn" on:click=toggle
                    style=move || format!("background:{}", TRACKS[idx.get()].color)>
                    {move || if playing.get() { "❚❚" } else { "▶" }}
                </button>
                <button class="transport-btn" on:click=go_next>"⏭"</button>
                <span style="flex:1"/>
                <span class="type-mono muted">{move || idx.get() + 1}"/" {TRACKS.len()}</span>
            </div>

            <div class="np-demo-note type-mono muted">
                "// демо-режим. загрузи .mp3-файл для воспроизведения."
            </div>
        </div>
    }.into_any()
}

#[component]
fn Scrubber(idx: RwSignal<usize>, cur: RwSignal<u32>, #[prop(into)] progress: Signal<f64>) -> AnyView {
    view! {
        <div class="np-scrubber">
            <div class="scrubber-track">
                <div class="scrubber-fill" style=move || format!("width:{}%", progress.get() * 100.0)/>
                <div class="scrubber-thumb" style=move || format!("left:{}%", progress.get() * 100.0)/>
            </div>
            <div class="scrubber-times">
                <span class="type-mono muted">{move || fmt_time(cur.get())}</span>
                <span class="type-mono muted">{move || fmt_time(TRACKS[idx.get()].dur)}</span>
            </div>
        </div>
    }.into_any()
}
