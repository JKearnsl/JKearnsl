use leptos::prelude::*;
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
        <section class="pt-[56px] pb-6">
            <div class="wrap">
                <div class="flex justify-between items-end gap-5 flex-wrap">
                    <div>
                        <div class="type-eyebrow">"// archive"</div>
                        <h2 class="h-section mt-2">"весь архив"</h2>
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
                                    <span class="inline-block size-[6px] rounded-full" style=format!("background:{}", color)/>
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
