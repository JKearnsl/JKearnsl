use leptos::prelude::*;

#[component]
pub fn Section() -> AnyView {
    view! {
        <section class="music-hero">
            <div class="wrap">
                <div class="type-eyebrow music-eyebrow">
                    <span class="eyebrow-line"/>
                    "музыка"
                </div>
                <div class="music-header">
                    <h1 class="h-section">
                        "что я "
                        <span class="italic-serif" style="color:var(--terracotta)">"пишу"</span>
                        <br/>
                        "между постами"
                    </h1>
                    <p class="music-desc italic-serif">
                        "Наброски, эмбиент и lo-fi, под которые удобно компилировать."
                    </p>
                </div>
            </div>
        </section>
    }.into_any()
}
