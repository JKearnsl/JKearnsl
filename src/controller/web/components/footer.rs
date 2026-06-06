use leptos::prelude::*;
use crate::controller::web::store::session::SessionStore;

#[component]
pub fn Footer() -> impl IntoView {
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("BUILD_DATE");
    let git_hash = env!("GIT_HASH");
    let commit_url = format!("https://github.com/JKearnsl/JKearnsl/commit/{git_hash}");

    let session = use_context::<SessionStore>().expect("SessionStore");

    view! {
        <footer class="border-t border-[var(--line)] bg-cream-2 pt-[72px] pb-[28px]">
            <div class="wrap">
                <div class="grid grid-cols-[1fr_auto] gap-[48px] items-end max-[900px]:grid-cols-1">
                    <div class="font-display font-bold text-[clamp(80px,14vw,220px)] tracking-[-0.05em] leading-[.85] text-ink">
                        "jkearnsl"
                        <span class="text-terracotta">"."</span>
                    </div>
                    <div class="grid grid-cols-3 gap-[36px] max-[900px]:grid-cols-2">
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
                <div class="mt-[32px] pt-[20px] border-t border-[var(--line)] flex justify-between items-center gap-6 flex-wrap font-mono text-[11px] text-muted">
                    <span>"© 2026 jkearnsl · cc-by 4.0 · сделано с помощью Rust + Leptos"</span>
                    <span class="inline-flex items-center gap-[2px]">
                        "v " {version} " / build " {build_date} " · "
                        <a
                            href=commit_url
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-terracotta underline decoration-dotted underline-offset-[2px] transition-colors hover:text-ochre"
                        >
                            {git_hash}
                        </a>
                        " · "
                        <Suspense fallback=move || view! {
                            <a href="/sign-in" class="text-terracotta underline decoration-dotted underline-offset-[2px] transition-colors hover:text-ochre">"войти"</a>
                        }>
                            {move || match session.get() {
                                Some(u) => view! {
                                    <a href="/control" class="text-terracotta underline decoration-dotted underline-offset-[2px] transition-colors hover:text-ochre">{u.username}</a>
                                }.into_any(),
                                None => view! {
                                    <a href="/sign-in" class="text-terracotta underline decoration-dotted underline-offset-[2px] transition-colors hover:text-ochre">"войти"</a>
                                }.into_any(),
                            }}
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
            <div class="type-eyebrow mb-[18px]">{title}</div>
            <ul class="list-none p-0 m-0 flex flex-col gap-[10px]">
                {items.into_iter().map(|(label, href)| view! {
                    <li>
                        <a href={href} class="font-mono text-[13px] text-ink inline-flex items-center gap-[6px]">
                            <span class="text-muted">"→"</span>
                            {label}
                        </a>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
}
