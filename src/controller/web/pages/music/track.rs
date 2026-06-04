use leptos::prelude::*;

#[component]
pub fn Art(color: &'static str, kind: &'static str, playing: bool) -> AnyView {
    view! {
        <div class="absolute inset-0 overflow-hidden" style=format!("background:{}", color)>
            <span class="absolute left-[14px] top-[12px] font-mono text-[11px] tracking-[.06em] uppercase text-cream opacity-85">"// "{kind}</span>
            <div class="absolute left-0 right-0 bottom-0 h-[44%] flex items-end gap-[2px] px-[14px] pb-[14px]">
                {(0..24usize).map(|i| {
                    let h = (0.35 + ((i * 7 + i * i) % 65) as f64 / 100.0) * 100.0;
                    let style = if playing {
                        let delay = format!("{:.2}s", (i % 12) as f64 * 0.06);
                        format!("height:{h:.0}%; animation: mwave 1.1s ease-in-out {delay} infinite alternate;")
                    } else {
                        format!("height:{h:.0}%")
                    };
                    view! { <span class="flex-1 bg-cream opacity-65 rounded-[1px] origin-bottom" style=style/> }.into_any()
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }.into_any()
}
