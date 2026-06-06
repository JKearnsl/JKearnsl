use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use crate::controller::web::lib::api::{session, users};
use crate::controller::web::components::ui::{
    button::Button,
    form_field::FormField,
};

const INPUT_CLASS: &str = "w-full bg-cream/40 border border-[var(--line)] rounded-[var(--radius-sm)] \
    py-[13px] px-4 font-sans text-[15px] text-ink outline-none \
    transition-[border-color,box-shadow] duration-200 \
    focus:border-terracotta \
    focus:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)] \
    placeholder:text-muted placeholder:opacity-60";

#[component]
pub fn Page() -> impl IntoView {
    let login_action = ServerAction::<session::Create>::new();
    let logout_action = ServerAction::<session::RemoveSelf>::new();
    let pending = login_action.pending();
    let login_result = login_action.value();
    let navigate = use_navigate();
    let navigate = StoredValue::new(navigate);

    let current_user = Resource::new(
        move || (login_action.version().get(), logout_action.version().get()),
        |_| users::get_self(),
    );

    Effect::new(move |_| {
        if let Some(Ok(())) = login_result.get() {
            navigate.with_value(|nav| nav("/control", Default::default()));
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(Some(_))) = current_user.get() {
            navigate.with_value(|nav| nav("/control", Default::default()));
        }
    });

    view! {
        <Title text="Войти"/>
        <main class="page min-h-[calc(100vh-72px)] flex items-center justify-center px-6 py-[60px] relative overflow-hidden">
            <div class="absolute -top-[240px] right-[8%] w-[500px] h-[500px] rounded-full bg-terracotta/[.12] blur-[110px] pointer-events-none"/>
            <div class="absolute -bottom-[220px] left-[4%] w-[440px] h-[440px] rounded-full bg-ochre/[.10] blur-[100px] pointer-events-none"/>
            <div class="w-full max-w-[460px] relative z-10">
                <div class="bg-paper/55 backdrop-blur-2xl border border-[var(--line)] rounded-[var(--radius)] pt-[52px] px-[44px] pb-[48px] max-[520px]:pt-[40px] max-[520px]:px-[28px] max-[520px]:pb-[36px] relative overflow-hidden shadow-[0_16px_64px_0_rgba(23,18,16,.09),inset_0_1px_0_rgba(255,255,255,.5)]">
                    <div class="absolute inset-x-0 top-0 h-px [background:linear-gradient(to_right,transparent,rgba(255,255,255,.65),transparent)]"/>
                    <div class="type-eyebrow mb-5 flex items-center gap-3">
                        <span class="inline-block w-[28px] h-px bg-current text-terracotta"/>
                        "// sign-in"
                    </div>
                    <Suspense fallback=move || view! {
                        <h1 class="h-card mb-2">"Вход"</h1>
                    }>
                        {move || current_user.get().map(|user| match user {
                            Ok(Some(username)) => view! {
                                <h1 class="h-card mb-2">
                                    "Привет, " {username.clone()} "."
                                </h1>
                                <p class="font-mono text-[12px] text-muted mb-8 leading-[1.6]">"Вы уже вошли в систему."</p>
                                <div class="flex flex-col gap-[18px]">
                                    <ActionForm action=logout_action>
                                        <Button submit=true>"Выйти →"</Button>
                                    </ActionForm>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <h1 class="h-card mb-2">"Вход"</h1>
                                <p class="font-mono text-[12px] text-muted mb-8 leading-[1.6]">"Введите данные для доступа к панели управления."</p>
                                <ActionForm action=login_action>
                                    <div class="flex flex-col gap-2">
                                        {move || login_result.get().map(|r| match r {
                                            Err(e) => view! {
                                                <p class="font-mono text-[12px] text-rust py-[10px] px-[14px] bg-rust/8 rounded-[var(--radius-sm)] border-l-2 border-rust">{e.to_string()}</p>
                                            }.into_any(),
                                            Ok(()) => view! { <span/> }.into_any(),
                                        })}
                                        <FormField label="Имя пользователя" label_for="username">
                                            <input
                                                type="text"
                                                id="username"
                                                name="username"
                                                class=INPUT_CLASS
                                                required
                                                autocomplete="username"
                                                placeholder="admin"
                                            />
                                        </FormField>
                                        <FormField label="Пароль" label_for="password">
                                            <input
                                                type="password"
                                                id="password"
                                                name="password"
                                                class=INPUT_CLASS
                                                required
                                                autocomplete="current-password"
                                                placeholder="••••••••"
                                            />
                                        </FormField>
                                        <Button submit=true class="w-full justify-center mt-2 py-[16px] px-[22px]" pending=pending>
                                            {move || if pending.get() { "Загрузка..." } else { "Войти →" }}
                                        </Button>
                                    </div>
                                </ActionForm>
                            }.into_any(),
                        })}
                    </Suspense>
                </div>
            </div>
        </main>
    }
}
