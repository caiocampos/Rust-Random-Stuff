fn normalize(c: char) -> Option<char> {
    if c.is_ascii_alphanumeric() {
        Some(c.to_ascii_lowercase())
    } else {
        None
    }
}

fn choose(o: Option<char>) -> char {
    if let Some(c) = o {
        c
    } else {
        ' '
    }
}

fn get_normalized(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    let normalized: Vec<char> = input.chars().filter_map(normalize).collect();
    let len = normalized.len() as f64;
    let x = len.sqrt().ceil() as usize;
    let y = (len / x as f64).ceil() as usize;
    (normalized.chunks(x).map(Vec::from).collect(), x, y)
}

pub fn encrypt(input: &str) -> String {
    if input.len() < 2 {
        return String::from(input);
    }
    let (normalized, x, y) = get_normalized(input);
    (0..x)
        .map(|i| {
            (0..y)
                .map(|j| choose(normalized[j].get(i).cloned()))
                .collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
