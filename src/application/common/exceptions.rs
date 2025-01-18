use std::collections::HashMap;
use thiserror::Error;


fn format_map(errors: &HashMap<String, String>) -> String {
    let mut entries = errors.iter().collect::<Vec<_>>();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let formatted_entries: Vec<String> = entries
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect();

    format!("[{}]", formatted_entries.join(", "))
}

#[derive(Error, Debug, Clone)]
pub enum ApplicationError {
    #[error("NotFound")]
    NotFound,
    #[error("ValidationError: {}", format_map(.0))]
    ValidationError(HashMap<String, String>),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden: You do not have permission to perform this action!")]
    Forbidden,
    #[error("UnexpectedError: {0}")]
    UnexpectedError(String),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_map() {
        let mut map = HashMap::new();
        map.insert("field1".to_string(), "error1".to_string());
        map.insert("field2".to_string(), "error2".to_string());

        let formatted = format_map(&map);
        assert_eq!(formatted, "[field1: error1, field2: error2]");
    }
}
