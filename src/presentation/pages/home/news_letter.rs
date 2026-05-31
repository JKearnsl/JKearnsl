use leptos::prelude::*;
use leptos_meta::Style;
use crate::presentation::components::ui::button::Button;
use crate::presentation::components::ui::input::Input;

#[component]
pub fn Section() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let sent = RwSignal::new(false);

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !email.get().is_empty() {
            sent.set(true);
        }
    };

    view! {
        <Style id="home-newsletter">{include_str!("./news_letter.css")}</Style>
        <section class="newsletter-section">
            <div class="wrap">
                <div class="newsletter-box">
                    <div class="nl-deco nl-deco-tr" style="color:var(--terracotta)">
                        <HalftoneDecoration rows=20 cols=20 max_r=4.0/>
                    </div>
                    <div class="nl-deco nl-deco-bl" style="color:var(--ochre)">
                        <HalftoneDecoration rows=18 cols=18 max_r=3.4/>
                    </div>
                    <div class="newsletter-left">
                        <div class="type-eyebrow" style="color:var(--ochre)">"// newsletter"</div>
                        <h2 class="h-section newsletter-title">"один пост — одно письмо"</h2>
                        <p class="newsletter-desc">
                            "Без расписания. Без маркетинга. Просто я отправляю письмо, \
                             когда выходит новый пост — иногда раз в неделю, иногда раз в месяц."
                        </p>
                    </div>
                    <div class="newsletter-right">
                        {move || if sent.get() {
                            view! {
                                <div class="newsletter-sent">
                                    <span class="sent-dot"/>
                                    <span class="type-mono-lg">"// готово. ждите первое письмо."</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <form class="newsletter-form" on:submit=submit>
                                    <Input
                                        value=email
                                        r#type="email"
                                        placeholder="ваш@email"
                                        class="newsletter-input"
                                        required=true
                                    />
                                    <Button submit=true>"подписаться"</Button>
                                </form>
                                <div class="nl-stats">
                                    <span>"~1 письмо / нед"</span>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </section>
    }.into_any()
}

#[component]
fn HalftoneDecoration(rows: usize, cols: usize, max_r: f64) -> impl IntoView {
    let circles: Vec<(f64, f64, f64)> = (0..rows)
        .flat_map(|y| {
            (0..cols).map(move |x| {
                let cx = (x as f64 + 0.5) / cols as f64;
                let cy = (y as f64 + 0.5) / rows as f64;
                let d = ((cx - 0.5).powi(2) + (cy - 0.5).powi(2)).sqrt() * 1.7;
                let f = (1.0_f64 - d).max(0.0);
                let r = f.max(0.4 / max_r) * max_r * 0.5;
                (cx * 100.0, cy * 100.0, r)
            })
        })
        .collect();

    view! {
        <svg viewBox="0 0 100 100" preserveAspectRatio="none" width="100%" height="100%" style="display:block">
            {circles.into_iter().map(|(cx, cy, r)| {
                let cx_s = format!("{:.2}", cx);
                let cy_s = format!("{:.2}", cy);
                let r_s  = format!("{:.2}", r);
                view! { <circle cx={cx_s} cy={cy_s} r={r_s} fill="currentColor"/> }
            }).collect_view()}
        </svg>
    }.into_any()
}
