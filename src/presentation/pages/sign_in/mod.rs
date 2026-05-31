use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use crate::presentation::api::{session, users};
use crate::presentation::components::ui::button::Button;
use crate::presentation::components::ui::form_field::FormField;

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
        <main class="page sign-in-page">
            <div class="sign-in-wrap">
                <div class="sign-in-card">
                    <div class="sign-in-card-deco"/>
                    <div class="type-eyebrow sign-in-eyebrow">
                        <span class="eyebrow-line"/>
                        "// /sign-in"
                    </div>
                    <Suspense fallback=move || view! {
                        <h1 class="h-card sign-in-title">"Вход"</h1>
                    }>
                        {move || current_user.get().map(|user| match user {
                            Ok(Some(username)) => view! {
                                <h1 class="h-card sign-in-title">
                                    "Привет, " {username.clone()} "."
                                </h1>
                                <p class="sign-in-hint">"Вы уже вошли в систему."</p>
                                <div class="sign-in-form">
                                    <ActionForm action=logout_action>
                                        <Button submit=true>"Выйти →"</Button>
                                    </ActionForm>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <h1 class="h-card sign-in-title">"Вход"</h1>
                                <p class="sign-in-hint">"Введите данные для доступа к панели управления."</p>
                                <div class="sign-in-form">
                                    <ActionForm action=login_action>
                                        <FormField label="Имя пользователя" label_for="username">
                                            <input
                                                type="text"
                                                id="username"
                                                name="username"
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
                                                required
                                                autocomplete="current-password"
                                                placeholder="••••••••"
                                            />
                                        </FormField>
                                        {move || login_result.get().map(|r| match r {
                                            Err(e) => view! {
                                                <p class="sign-in-error">{e.to_string()}</p>
                                            }.into_any(),
                                            Ok(()) => view! { <span/> }.into_any(),
                                        })}
                                        <Button submit=true class="sign-in-btn" pending=pending>
                                            {move || if pending.get() { "Загрузка..." } else { "Войти →" }}
                                        </Button>
                                    </ActionForm>
                                </div>
                            }.into_any(),
                        })}
                    </Suspense>
                </div>
            </div>
        </main>
    }
}
