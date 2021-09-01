/// Cipher a character with the Atbash cipher.
fn encode_char(c: char) -> Option<char> {
    if c.is_numeric() {
        Some(c)
    } else if c.is_ascii_alphabetic() {
        Some((b'z' - ((c as u8).to_ascii_lowercase() - b'a')) as char)
    } else {
        None
    }
}

/// Cipher a text with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(encode_char)
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 5 == 0 {
                vec![' ', c]
            } else {
                vec![c]
            }
        })
        .collect()
}

/// Decipher a text with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(encode_char).collect()
}
