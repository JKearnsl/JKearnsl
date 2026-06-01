use leptos::prelude::*;
use super::{TRACKS, fmt_time, track};

#[component]
pub fn Section(idx: RwSignal<usize>, playing: RwSignal<bool>) -> AnyView {
    view! {
        <section class="pt-[40px] pb-[88px]">
            <div class="wrap">
                <div class="type-eyebrow">"// все дорожки"</div>
                <div class="border-t border-[var(--line)]">
                    {TRACKS.iter().enumerate().map(|(i, t)| {
                        let is_active = move || idx.get() == i;
                        view! {
                            <button
                                class="track-row"
                                class:track-active=is_active
                                on:click=move |_| { idx.set(i); playing.set(true); }
                            >
                                <span class="font-mono text-[13px]" style=move || {
                                    if is_active() { format!("color:{}", TRACKS[idx.get()].color) }
                                    else { "color:var(--muted)".to_string() }
                                }>
                                    {move || if is_active() && playing.get() { "♪".to_string() } else { t.no.to_string() }}
                                </span>
                                <span class="relative w-[56px] h-[42px] border border-ink overflow-hidden block">
                                    <track::Art color=t.color kind=t.kind playing=false/>
                                </span>
                                <span class="min-w-0 flex flex-col gap-[2px]">
                                    <span class="font-display font-semibold text-[18px] tracking-[-0.01em] text-ink block text-pretty">{t.title}</span>
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
