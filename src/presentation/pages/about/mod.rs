use leptos::prelude::*;
use crate::presentation::components::ui::button::{Button, Variant as BtnVariant};

#[component]
pub fn Page() -> impl IntoView {
    let stack = ["Rust", "Haskell", "Python", "TypeScript", "OCaml", "LaTeX", "Vim", "Linux"];
    let reads = [
        ("Pierce — Types and Programming Languages", "// книга"),
        ("Lawvere — Conceptual Mathematics", "// книга"),
        ("Pearl — Causality", "// книга"),
        ("Bartosz Milewski — Category Theory for Programmers", "// курс"),
    ];

    view! {
        <main class="page">
            <section class="about-hero-section">
                <div class="wrap">
                    <div class="type-eyebrow about-eyebrow">
                        <span class="eyebrow-line"/>
                        "// /about"
                    </div>
                    <div class="about-grid">
                        <div class="about-left">
                            <h1 class="h-hero about-headline">
                                "jkearnsl"
                                <br/>
                                <span class="italic-serif" style="color:var(--terracotta)">"пишет,"</span>
                                <br/>
                                "чтобы понять"
                            </h1>
                            <p class="about-bio">
                                "Привет. Я JKearnsl. Пишу здесь про языки программирования, доказательства,
                                фундаментальную физику и то, что меня в данный момент не отпускает.
                                Иногда это категорная теория. Иногда — почему сон важнее всего остального.
                                Цель блога одна: писать так, чтобы я сам через год не понял половину,
                                прочёл и обрадовался."
                            </p>
                            <div class="about-actions">
                                <Button href="/">"← к постам"</Button>
                                <Button variant=BtnVariant::Ghost href="https://github.com/JKearnsl" target="_blank">"github"</Button>
                            </div>
                        </div>
                        <div class="about-portrait">
                            <div class="portrait-card">
                                <div class="portrait-inner" style="background:var(--terracotta)">
                                    <div class="portrait-letters">"JK"</div>
                                    <div class="portrait-meta-tl type-mono">"// автопортрет"</div>
                                    <div class="portrait-meta-br type-mono">"RU · 2026"</div>
                                </div>
                                <div class="portrait-dot"/>
                                <div class="portrait-terminal type-mono">
                                    <span style="color:var(--ochre)">">"</span>
                                    " hi.world()"
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="about-stats">
                <div class="wrap stats-row">
                    <div class="stat-item">
                        <div class="stat-num" style="color:var(--terracotta)">"∞"</div>
                        <div class="type-mono muted">"постов"</div>
                    </div>
                    <div class="stat-item stat-border">
                        <div class="stat-num">"3"</div>
                        <div class="type-mono muted">"темы"</div>
                    </div>
                    <div class="stat-item stat-border">
                        <div class="stat-num" style="color:var(--ochre)">"∀"</div>
                        <div class="type-mono muted">"подписчиков"</div>
                    </div>
                    <div class="stat-item stat-border">
                        <div class="stat-num">"2026"</div>
                        <div class="type-mono muted">"начат"</div>
                    </div>
                </div>
            </section>

            <section class="about-details">
                <div class="wrap about-details-grid">
                    <div class="about-stack">
                        <div class="type-eyebrow">"// стек"</div>
                        <h2 class="h-section">"чем пользуюсь"</h2>
                        <div class="stack-chips">
                            {stack.iter().map(|s| view! {
                                <span class="stack-chip type-mono-lg">{*s}</span>
                            }).collect_view()}
                        </div>
                        <div class="reading-block">
                            <div class="type-mono muted">"// сейчас читаю"</div>
                            <div class="italic-serif reading-text">
                                "«Категорная теория для программистов» — Bartosz Milewski"
                            </div>
                        </div>
                    </div>

                    <div class="about-reads">
                        <div class="type-eyebrow">"// что повлияло"</div>
                        <h2 class="h-section">"источники"</h2>
                        <ul class="reads-list">
                            {reads.iter().enumerate().map(|(i, (title, kind))| view! {
                                <li class="read-item">
                                    <span class="read-no type-mono muted">
                                        {format!("{:02}", i + 1)}
                                    </span>
                                    <span class="read-title">{*title}</span>
                                    <span class="read-kind type-mono muted">{*kind}</span>
                                </li>
                            }).collect_view()}
                        </ul>
                    </div>
                </div>
            </section>
        </main>
    }
}
