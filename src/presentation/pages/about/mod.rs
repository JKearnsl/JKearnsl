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
            <section class="pt-[56px] pb-[40px]">
                <div class="wrap">
                    <div class="type-eyebrow flex items-center gap-[14px] mb-5">
                        <span class="eyebrow-line"/>
                        "// /about"
                    </div>
                    <div class="grid grid-cols-[1.4fr_1fr] gap-[64px] items-start max-[900px]:grid-cols-1">
                        <div class="about-left">
                            <h1 class="h-hero leading-[.92]">
                                "jkearnsl"
                                <br/>
                                <span class="italic-serif" style="color:var(--terracotta)">"пишет,"</span>
                                <br/>
                                "чтобы понять"
                            </h1>
                            <p class="text-[20px] text-ink-2 max-w-[560px] mt-8 leading-[1.55]">
                                "Привет. Я JKearnsl. Пишу здесь про языки программирования, доказательства,
                                фундаментальную физику и то, что меня в данный момент не отпускает.
                                Иногда это категорная теория. Иногда — почему сон важнее всего остального.
                                Цель блога одна: писать так, чтобы я сам через год не понял половину,
                                прочёл и обрадовался."
                            </p>
                            <div class="flex gap-3 mt-8 flex-wrap">
                                <Button href="/">"← к постам"</Button>
                                <Button variant=BtnVariant::Ghost href="https://github.com/JKearnsl" target="_blank">"github"</Button>
                            </div>
                        </div>
                        <div class="relative aspect-[1/1.1]">
                            <div class="absolute inset-[6%]">
                                <div class="absolute inset-0 rounded-[24px] overflow-hidden" style="background:var(--terracotta)">
                                    <div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-[52%] font-display font-bold text-[clamp(120px,18vw,260px)] text-cream tracking-[-0.07em] leading-[.85]">"JK"</div>
                                    <div class="absolute left-6 top-6 font-mono text-[11px] text-cream opacity-85 type-mono">"// автопортрет"</div>
                                    <div class="absolute right-6 bottom-5 font-mono text-[11px] text-cream opacity-85 type-mono">"RU · 2026"</div>
                                </div>
                                <div class="absolute right-[-4%] top-[-2%] w-[42%] aspect-square rounded-full bg-ochre border-2 border-ink"/>
                                <div class="absolute left-[-2%] bottom-[-2%] bg-ink text-cream py-[14px] px-[18px] rounded-[14px] font-mono text-[12px] type-mono">
                                    <span style="color:var(--ochre)">">"</span>
                                    " hi.world()"
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-[40px] border-t border-b border-[var(--line)]">
                <div class="wrap grid grid-cols-4 gap-5 max-[900px]:grid-cols-2">
                    <div class="flex flex-col gap-[6px]">
                        <div class="font-display font-semibold text-[clamp(48px,5vw,72px)] tracking-[-0.04em] leading-none text-ink" style="color:var(--terracotta)">"∞"</div>
                        <div class="type-mono muted">"постов"</div>
                    </div>
                    <div class="flex flex-col gap-[6px] border-l border-[var(--line)] pl-[28px]">
                        <div class="font-display font-semibold text-[clamp(48px,5vw,72px)] tracking-[-0.04em] leading-none text-ink">"3"</div>
                        <div class="type-mono muted">"темы"</div>
                    </div>
                    <div class="flex flex-col gap-[6px] border-l border-[var(--line)] pl-[28px]">
                        <div class="font-display font-semibold text-[clamp(48px,5vw,72px)] tracking-[-0.04em] leading-none text-ink" style="color:var(--ochre)">"∀"</div>
                        <div class="type-mono muted">"подписчиков"</div>
                    </div>
                    <div class="flex flex-col gap-[6px] border-l border-[var(--line)] pl-[28px]">
                        <div class="font-display font-semibold text-[clamp(48px,5vw,72px)] tracking-[-0.04em] leading-none text-ink">"2026"</div>
                        <div class="type-mono muted">"начат"</div>
                    </div>
                </div>
            </section>

            <section class="py-[80px]">
                <div class="wrap grid grid-cols-2 gap-[64px] max-[900px]:grid-cols-1">
                    <div class="about-stack">
                        <div class="type-eyebrow">"// стек"</div>
                        <h2 class="h-section">"чем пользуюсь"</h2>
                        <div class="flex gap-[10px] flex-wrap mt-7">
                            {stack.iter().map(|s| view! {
                                <span class="py-[10px] px-[16px] rounded-[12px] bg-paper border border-[var(--line)] font-mono text-[13px] type-mono-lg">{*s}</span>
                            }).collect_view()}
                        </div>
                        <div class="mt-[40px] p-6 bg-cream-2 border border-[var(--line)] rounded-[18px]">
                            <div class="type-mono muted">"// сейчас читаю"</div>
                            <div class="italic-serif mt-2 font-serif italic text-[22px] leading-[1.3]">
                                "«Категорная теория для программистов» — Bartosz Milewski"
                            </div>
                        </div>
                    </div>

                    <div class="about-reads">
                        <div class="type-eyebrow">"// что повлияло"</div>
                        <h2 class="h-section">"источники"</h2>
                        <ul class="list-none p-0 mt-7 flex flex-col">
                            {reads.iter().enumerate().map(|(i, (title, kind))| view! {
                                <li class="grid grid-cols-[40px_1fr_auto] items-center py-[18px] border-t border-[var(--line)] gap-3 last:border-b last:border-[var(--line)]">
                                    <span class="read-no type-mono muted">
                                        {format!("{:02}", i + 1)}
                                    </span>
                                    <span class="font-display text-[18px]">{*title}</span>
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
