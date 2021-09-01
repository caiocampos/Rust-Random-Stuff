use rand::Rng;

const A: u8 = b'a';
const Z: u8 = b'z';
const MAX: u8 = 26;

pub fn encode(key: &str, s: &str) -> Option<String> {
    cipher(key, s, false)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    cipher(key, s, true)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_key();
    let encoded = cipher_unchecked(key.as_str(), s, false);
    (key, encoded)
}

fn generate_key() -> String {
    let mut generator = rand::thread_rng();
    (0..100)
        .map(|_| generator.gen_range(A, Z) as char)
        .collect()
}

fn cipher(key: &str, text: &str, inverse: bool) -> Option<String> {
    match !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase()) {
        true => Some(cipher_unchecked(key, text, inverse)),
        _ => None,
    }
}

fn cipher_unchecked(key: &str, text: &str, inverse: bool) -> String {
    text.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| to_char((val(c) + key_factor(k, inverse)) % MAX))
        .collect()
}

fn val(code: char) -> u8 {
    code as u8 - A
}

fn to_char(code: u8) -> char {
    (code + A) as char
}

fn key_factor(code: char, inverse: bool) -> u8 {
    match inverse {
        true => MAX - val(code),
        _ => val(code),
    }
}
