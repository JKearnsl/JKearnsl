use leptos::prelude::*;
use crate::controller::web::components::ui::{
    button::Button,
    input::Input,
};

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
        <section class="py-[80px]">
            <div class="wrap">
                <div class="bg-ink text-cream rounded-[28px] p-[56px] grid grid-cols-[1.2fr_1fr] gap-[40px] items-center relative overflow-hidden max-[760px]:grid-cols-1 max-[760px]:p-[36px_28px]">
                    <div class="absolute pointer-events-none -right-[40px] -top-[40px] w-[260px] h-[260px] opacity-85" style="color:var(--terracotta)">
                        <HalftoneDecoration rows=20 cols=20 max_r=4.0/>
                    </div>
                    <div class="absolute pointer-events-none -left-[100px] -bottom-[100px] w-[240px] h-[240px] opacity-60" style="color:var(--ochre)">
                        <HalftoneDecoration rows=18 cols=18 max_r=3.4/>
                    </div>
                    <div class="relative">
                        <div class="type-eyebrow" style="color:var(--ochre)">"// newsletter"</div>
                        <h2 class="h-section mt-3 text-cream">"один пост — одно письмо"</h2>
                        <p class="text-cream/70 max-w-[380px] mt-[18px] text-[16px]">
                            "Без расписания. Без маркетинга. Просто я отправляю письмо, \
                             когда выходит новый пост — иногда раз в неделю, иногда раз в месяц."
                        </p>
                    </div>
                    <div class="relative">
                        {move || if sent.get() {
                            view! {
                                <div class="py-[22px] px-[22px] border border-ochre rounded-[18px] flex items-center gap-[14px]">
                                    <span class="inline-block size-3 rounded-full bg-ochre"/>
                                    <span class="type-mono-lg">"// готово. ждите первое письмо."</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <form class="flex bg-cream text-ink p-[6px] rounded-full" on:submit=submit>
                                    <Input
                                        value=email
                                        r#type="email"
                                        placeholder="ваш@email"
                                        class="flex-1 border-none bg-transparent outline-none py-[14px] px-[22px] font-mono text-[13px] text-ink"
                                        required=true
                                    />
                                    <Button submit=true>"подписаться"</Button>
                                </form>
                                <div class="flex gap-[18px] mt-[18px] font-mono text-[11px] text-cream/50">
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
