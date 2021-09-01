/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (mut count, mut sum) = (0u8, 0u8);
    let valid = code.chars().rev().all(|el| match el {
        '0'..='9' => {
            sum += calculates(el as u8 - b'0', is_odd(count));
            count += 1;
            true
        }
        ' ' => true,
        _ => false,
    });
    match count {
        0 | 1 => false, // small code
        _ => valid && (sum % 10 == 0),
    }
}

fn is_odd(n: u8) -> bool {
    n & 1 == 1
}

fn calculates(n: u8, double: bool) -> u8 {
    match (double, n > 4) {
        (true, true) => n * 2 - 9, // if val > 4 then val * 2 > 9
        (true, false) => n * 2,
        _ => n,
    }
}
