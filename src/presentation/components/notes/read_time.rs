

pub fn calc_mins(body: &str) -> usize {
    let words = body.split_whitespace().count();
    (words / 200).max(1)
}
