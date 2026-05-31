use leptos::prelude::*;
use crate::presentation::components::ui::button::Button;

#[component]
pub fn Page() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <main class="page not-found-page">
            <div class="wrap">
                <div class="not-found-inner">
                    <div class="type-eyebrow">"// 404"</div>
                    <h1 class="not-found-title">
                        "4"<span>"0"</span>"4"
                    </h1>
                    <p class="not-found-msg">
                        "страница не найдена. возможно, она переехала или никогда не существовала."
                    </p>
                    <Button href="/">"← вернуться домой"</Button>
                </div>
            </div>
        </main>
    }
}
