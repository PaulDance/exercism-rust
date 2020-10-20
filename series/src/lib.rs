/// Returns a vector of windows of length `len` in the given string of `digits`.
pub fn series(digits: &str, len: usize) -> Vec<String> {
    // Don't know how +1 would would make sense here.
    if len == 0 {
        vec![String::new(); digits.len() + 1]
    } else {
        digits
            .as_bytes()
            .windows(len)
            .map(|series| String::from_utf8(series.to_vec()).unwrap())
            .collect()
    }
}
