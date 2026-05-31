mod hero;
mod player;
mod list;
mod track;

use leptos::prelude::*;
use leptos_meta::Style;

pub struct Track {
    pub no: &'static str,
    pub title: &'static str,
    pub dur: u32,
    pub kind: &'static str,
    pub color: &'static str,
}

pub const TRACKS: &[Track] = &[
    Track { no: "01", title: "Ночной компилятор",    dur: 212, kind: "ambient", color: "var(--terracotta)" },
    Track { no: "02", title: "Borrow checker blues", dur: 178, kind: "lo-fi",   color: "var(--ochre)" },
    Track { no: "03", title: "ζ(s) = 0",             dur: 241, kind: "idm",     color: "var(--plum)" },
    Track { no: "04", title: "Глимфатическая",       dur: 196, kind: "ambient", color: "var(--terracotta)" },
    Track { no: "05", title: "P ≠ NP · extended",    dur: 305, kind: "techno",  color: "var(--ochre)" },
    Track { no: "06", title: "Эндофунктор",          dur: 154, kind: "lo-fi",   color: "var(--plum)" },
];

pub fn fmt_time(secs: u32) -> String {
    let m = secs / 60;
    let s = secs % 60;
    format!("{m}:{s:02}")
}

#[component]
pub fn Page() -> impl IntoView {
    let idx = RwSignal::new(0usize);
    let playing = RwSignal::new(false);
    let cur = RwSignal::new(0u32);
    Effect::new(move |_| { let _ = idx.get(); cur.set(0); });

    view! {
        <Style id="page-music">{include_str!("./music.css")}</Style>
        <main class="page">
            <hero::Section/>
            <player::Section idx playing cur/>
            <list::Section idx playing/>
        </main>
    }
}
