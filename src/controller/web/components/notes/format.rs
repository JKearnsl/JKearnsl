
pub fn date(dt: &chrono::DateTime<chrono::Utc>) -> String {
    use chrono::Datelike;
    format!("{} / {:02} / {:02}", dt.year(), dt.month(), dt.day())
}
