use leptos::prelude::*;
use leptos_meta::Style;
use crate::presentation::api::users::get_self;


#[component]
pub fn Footer() -> impl IntoView {
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("BUILD_DATE");
    let git_hash = env!("GIT_HASH");
    let commit_url = format!("https://github.com/JKearnsl/JKearnsl/commit/{git_hash}");

    let current_user = Resource::new(|| (), |_| get_self());

    view! {
        <Style id="comp-footer">{include_str!("./footer.css")}</Style>
        <footer class="site-footer">
            <div class="wrap">
                <div class="footer-grid">
                    <div class="footer-wordmark">
                        "jkearnsl"
                        <span class="footer-dot">"."</span>
                    </div>
                    <div class="footer-cols">
                        <Col title="// архив" items=vec![
                            ("программирование", "/"),
                            ("математика", "/"),
                            ("наука", "/"),
                        ]/>
                        <Col title="// связаться" items=vec![
                            ("github", "https://github.com/JKearnsl"),
                            ("email", "mailto:jkearnsl@example.com"),
                            ("rss", "/rss.xml"),
                        ]/>
                        <Col title="// блог" items=vec![
                            ("об авторе", "/about"),
                            ("музыка", "/music"),
                            ("галерея", "/gallery"),
                        ]/>
                    </div>
                </div>
                <div class="footer-meta">
                    <span>"© 2026 jkearnsl · cc-by 4.0 · сделано с помощью Rust + Leptos"</span>
                    <span class="footer-build">
                        "v " {version} " / build " {build_date} " · "
                        <a
                            href=commit_url
                            target="_blank"
                            rel="noopener noreferrer"
                            class="footer-hash-link"
                        >
                            {git_hash}
                        </a>
                        " · "
                        <Suspense fallback=move || view! {
                            <a href="/sign-in" class="footer-hash-link">"войти"</a>
                        }>
                            {move || current_user.get().map(|user| match user {
                                Ok(Some(username)) => view! {
                                    <a href="/control" class="footer-hash-link">{username}</a>
                                }.into_any(),
                                _ => view! {
                                    <a href="/sign-in" class="footer-hash-link">"войти"</a>
                                }.into_any(),
                            })}
                        </Suspense>
                    </span>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn Col(title: &'static str, items: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div class="footer-col">
            <div class="type-eyebrow footer-col-title">{title}</div>
            <ul class="footer-col-list">
                {items.into_iter().map(|(label, href)| view! {
                    <li>
                        <a href={href} class="footer-col-link">
                            <span class="footer-arrow">"→"</span>
                            {label}
                        </a>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
}
