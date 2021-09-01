/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (mut count, mut sum_even, mut sum_odd, mut even) = (0u8, 0u8, 0u8, true);
    for el in code.chars() {
        match el {
            '0'..='9' => {
                let val = el as u8 - b'0';
                sum_even += calculates(val, even);
                sum_odd += calculates(val, !even);
                even = !even;
                count += 1;
            }
            ' ' => (),
            _ => return false, // invalid character
        }
    }
    match count {
        0 | 1 => false,                            // small code
        _ if count & 1 == 0 => sum_even % 10 == 0, // uses sum_even if count is even
        _ => sum_odd % 10 == 0,                    // uses sum_odd if count is odd
    }
}

fn calculates(n: u8, double: bool) -> u8 {
    match (double, n > 4) {
        // if val > 4 then val * 2 > 9
        (true, true) => (n << 1) - 9, // val * 2 - 9
        (true, false) => (n << 1),    // val * 2
        _ => n,
    }
}
