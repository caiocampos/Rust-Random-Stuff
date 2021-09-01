pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        Vec::new()
    } else {
        (0..=digits.len() - len)
            .map(|p| digits[p..p + len].to_string())
            .collect()
    }
}
