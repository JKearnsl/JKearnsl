use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Prog,
    Math,
    Science,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Prog => "prog",
            Category::Math => "math",
            Category::Science => "science",
        }
    }
}

impl TryFrom<&str> for Category {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "prog" => Ok(Category::Prog),
            "math" => Ok(Category::Math),
            "science" => Ok(Category::Science),
            _ => Err(format!("unknown category: {}", s)),
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
