pub fn rotate(input: &str, key: i8) -> String {
    if key % 26 == 0 {
        return input.into();
    }
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let e = (c as i32 + key as i32) as u8;
                if (e > b'z') || (c < 'a' && e > b'Z') {
                    (e - 26) as char
                } else {
                    e as char
                }
            } else {
                c
            }
        })
        .collect()
}
