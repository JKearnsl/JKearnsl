use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum State {
    Published,
    Draft,
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Published => "Published",
            State::Draft => "Draft",
        }
    }
}

impl TryFrom<&str> for State {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Published" => Ok(State::Published),
            "Draft" => Ok(State::Draft),
            _ => Err(format!("unknown state: {}", s)),
        }
    }
}
