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
    let mut cipher = String::new();
    let mut counter = 0;
    for c in plain.chars() {
        if counter == 5 {
            cipher.push(' ');
            counter = 0;
        }
        if let Some(e) = encode_char(c) {
            cipher.push(e);
            counter += 1;
        }
    }
    cipher.trim_end().to_owned()
}

/// Decipher a text with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(encode_char).collect()
}
