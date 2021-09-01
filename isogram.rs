pub fn check(candidate: &str) -> bool {
    check_vb(candidate)
}

pub fn check_va(candidate: &str) -> bool {
    let mut s = String::from(candidate);
    s.retain(char::is_alphabetic);
    s.make_ascii_uppercase();
    !s.chars()
        .any(|c| s.matches(c).collect::<Vec<&str>>().len() > 1)
}

pub fn check_vb(candidate: &str) -> bool {
    let s = candidate.to_uppercase();
    !s.chars()
        .any(|c| c.is_alphabetic() && s.matches(c).collect::<Vec<&str>>().len() > 1)
}
