use leptos::prelude::*;
use crate::controller::web::components::ui::button::Button;

#[component]
pub fn Page() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <main class="page min-h-[70vh] flex items-center">
            <div class="wrap">
                <div class="py-20 flex flex-col gap-6">
                    <div class="type-eyebrow">"// 404"</div>
                    <h1 class="text-[clamp(100px,20vw,200px)] tracking-[-0.05em] leading-[.85] text-ink font-display font-bold">
                        "4"<span class="text-terracotta">"0"</span>"4"
                    </h1>
                    <p class="text-[20px] text-ink-2 max-w-[480px]">
                        "страница не найдена. возможно, она переехала или никогда не существовала."
                    </p>
                    <Button href="/">"← вернуться домой"</Button>
                </div>
            </div>
        </main>
    }
}
