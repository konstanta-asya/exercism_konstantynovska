pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return Vec::new();
    }
    (0..=digits.len() - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
