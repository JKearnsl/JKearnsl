use leptos::prelude::*;
use leptos_meta::Style;
use crate::presentation::components::notes::category;
use crate::presentation::components::ui::chip::{Chip, ChipRow};


#[component]
pub fn Bar(filter: RwSignal<String>) -> impl IntoView {
    let categories = [
        ("all", "все"),
        ("prog", "программирование"),
        ("math", "математика"),
        ("science", "наука"),
    ];

    view! {
        <Style id="home-filter">{include_str!("./filter.css")}</Style>
        <section class="filter-section">
            <div class="wrap">
                <div class="section-header">
                    <div>
                        <div class="type-eyebrow">"// archive"</div>
                        <h2 class="h-section">"весь архив"</h2>
                    </div>
                    <ChipRow>
                        {categories.into_iter().map(|(id, label)| {
                            let id_str = id.to_string();
                            let id_str2 = id_str.clone();
                            let color = category::color_str(id);
                            view! {
                                <Chip
                                    active=move || filter.get() == id_str
                                    on_click=move |_| filter.set(id_str2.clone())
                                >
                                    <span class="chip-dot" style=format!("background:{}", color)/>
                                    {label}
                                </Chip>
                            }
                        }).collect_view()}
                    </ChipRow>
                </div>
            </div>
        </section>
    }
}
