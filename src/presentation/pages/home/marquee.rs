use leptos::prelude::*;
use leptos_meta::Style;


#[component]
pub fn Section() -> impl IntoView {
    let items = [
        "хорошие идеи", "плохие компиляторы", "теорема Гёделя",
        "Bell-неравенство", "ζ(s)", "категорная теория", "thermodynamics",
        "P≠NP", "rust", "category theory", "neutrino", "fourier", "монады",
    ];
    let seq: Vec<_> = items.iter().chain(items.iter()).enumerate().collect();

    view! {
        <Style id="home-marquee">{include_str!("./marquee.css")}</Style>
        <section class="marquee-section">
            <div class="marquee">
                {seq.into_iter().map(|(i, t)| {
                    let color_cls = match i % 3 {
                        0 => "mc1",
                        1 => "mc2",
                        _ => "mc3",
                    };
                    view! {
                        <span class="marquee-item">
                            <span class={color_cls}>{*t}</span>
                            <span class="marquee-dot"/>
                        </span>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
