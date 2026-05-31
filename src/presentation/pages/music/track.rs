use leptos::prelude::*;

#[component]
pub fn Art(color: &'static str, kind: &'static str, playing: bool) -> AnyView {
    view! {
        <div class="track-art" style=format!("background:{}", color)>
            <span class="track-kind type-mono">"// "{kind}</span>
            <div class="wave-bars">
                {(0..24usize).map(|i| {
                    let h = (0.35 + ((i * 7 + i * i) % 65) as f64 / 100.0) * 100.0;
                    let style = if playing {
                        let delay = format!("{:.2}s", (i % 12) as f64 * 0.06);
                        format!("height:{h:.0}%; animation: mwave 1.1s ease-in-out {delay} infinite alternate;")
                    } else {
                        format!("height:{h:.0}%")
                    };
                    view! { <span class="wave-bar" style=style/> }.into_any()
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }.into_any()
}
