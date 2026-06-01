use leptos::prelude::*;

#[component]
pub fn Section() -> impl IntoView {
    let items = [
        "хорошие идеи", "плохие компиляторы", "теорема Гёделя",
        "Bell-неравенство", "ζ(s)", "категорная теория", "thermodynamics",
        "P≠NP", "rust", "category theory", "neutrino", "fourier", "монады",
    ];
    let seq: Vec<_> = items.iter().chain(items.iter()).enumerate().collect();

    view! {
        <section class="border-t border-b border-[var(--line)] py-6 mt-8 overflow-hidden bg-cream-2">
            <div class="marquee">
                {seq.into_iter().map(|(i, t)| {
                    let color_cls = match i % 3 {
                        0 => "text-terracotta",
                        1 => "text-ochre",
                        _ => "text-ink",
                    };
                    view! {
                        <span class="inline-flex items-center gap-6">
                            <span class={color_cls}>{*t}</span>
                            <span class="inline-block size-[14px] rounded-full bg-ink"/>
                        </span>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
