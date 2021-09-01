/// Cipher a character with the Atbash cipher.
fn encode_char(c: char) -> char {
    (b'z' - ((c as u8).to_ascii_lowercase() - b'a')) as char
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
        if c.is_numeric() {
            cipher.push(c);
            counter += 1;
        } else if c.is_ascii_alphabetic() {
            cipher.push(encode_char(c));
            counter += 1;
        }
    }
    cipher.trim_end().to_owned()
}

/// Decipher a text with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut plain = String::new();
    for c in cipher.chars() {
        if c.is_numeric() {
            plain.push(c);
        } else if c.is_ascii_alphabetic() {
            plain.push(encode_char(c));
        }
    }
    plain
}
