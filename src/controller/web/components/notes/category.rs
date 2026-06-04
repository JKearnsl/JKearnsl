use crate::domain::models::note::Category;

pub fn label(cat: &Category) -> &'static str {
    match cat {
        Category::Prog => "программирование",
        Category::Math => "математика",
        Category::Science => "наука",
    }
}

pub fn color(cat: &Category) -> &'static str {
    match cat {
        Category::Prog => "var(--terracotta)",
        Category::Math => "var(--ochre)",
        Category::Science => "var(--plum)",
    }
}

pub fn color_str(s: &str) -> &'static str {
    match s {
        "prog" => "var(--terracotta)",
        "math" => "var(--ochre)",
        "science" => "var(--plum)",
        _ => "var(--ink)",
    }
}
