pub fn abbreviate(phrase: &str) -> String {
    if phrase.len() == 0 {
        return String::new();
    }
    let mut vec: Vec<char> = Vec::new();
    vec.push(' ');
    vec.extend(phrase.chars());
    vec.windows(2).filter_map(|chars| {
        match chars {
            [a,b] if !a.is_alphabetic() && b.is_alphabetic() => Some(b.to_ascii_uppercase()),
            [a,b] if a.is_lowercase() && b.is_uppercase() => Some(b.clone()),
            _ => None
        }
    }).collect()
}
