use leptos::prelude::*;
use super::{TRACKS, fmt_time, track};

#[component]
pub fn Section(idx: RwSignal<usize>, playing: RwSignal<bool>) -> AnyView {
    view! {
        <section class="track-list-section">
            <div class="wrap">
                <div class="type-eyebrow">"// все дорожки"</div>
                <div class="track-list">
                    {TRACKS.iter().enumerate().map(|(i, t)| {
                        let is_active = move || idx.get() == i;
                        view! {
                            <button
                                class="track-row"
                                class:track-active=is_active
                                on:click=move |_| { idx.set(i); playing.set(true); }
                            >
                                <span class="track-no type-mono" style=move || {
                                    if is_active() { format!("color:{}", TRACKS[idx.get()].color) }
                                    else { "color:var(--muted)".to_string() }
                                }>
                                    {move || if is_active() && playing.get() { "♪".to_string() } else { t.no.to_string() }}
                                </span>
                                <span class="track-art-mini">
                                    <track::Art color=t.color kind=t.kind playing=false/>
                                </span>
                                <span class="track-info">
                                    <span class="track-title">{t.title}</span>
                                    <span class="type-mono muted">"JKearnsl"</span>
                                </span>
                                <span class="type-mono" style=format!("color:{}", t.color)>{t.kind}</span>
                                <span class="type-mono muted">{fmt_time(t.dur)}</span>
                            </button>
                        }.into_any()
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }.into_any()
}
