pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '(' | '[' => vec.push(c),
            '}' | ')' | ']' => {
                match (c, vec.last()) {
                    ('}', Some('{')) | (')', Some('(')) | (']', Some('[')) => vec.pop(),
                    _ => return false,
                };
            }
            _ => continue,
        }
    }
    vec.len() == 0
}
