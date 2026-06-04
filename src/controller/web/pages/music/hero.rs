use leptos::prelude::*;

#[component]
pub fn Section() -> AnyView {
    view! {
        <section class="pt-[56px] pb-8">
            <div class="wrap">
                <div class="type-eyebrow flex items-center gap-3">
                    <span class="inline-block w-[36px] h-px bg-current"/>
                    "музыка"
                </div>
                <div class="flex justify-between items-end gap-6 flex-wrap mt-[18px]">
                    <h1 class="h-section">
                        "что я "
                        <span class="italic-serif" style="color:var(--terracotta)">"пишу"</span>
                        <br/>
                        "между постами"
                    </h1>
                    <p class="font-serif italic text-[clamp(18px,1.6vw,23px)] text-ink-2 max-w-[380px] leading-[1.4] pb-2 italic-serif">
                        "Наброски, эмбиент и lo-fi, под которые удобно компилировать."
                    </p>
                </div>
            </div>
        </section>
    }.into_any()
}
