/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (mut count, mut sum_even, mut sum_odd) = (0u8, 0u8, 0u8);
    for el in code.chars() {
        match el {
            '0'..='9' => {
                let (val, even) = (el as u8 - b'0', is_even(count));
                sum_even += calculates(val, even);
                sum_odd += calculates(val, !even);
                count += 1;
            }
            ' ' => (),
            _ => return false, // invalid character
        }
    }
    match count {
        0 | 1 => false,                            // small code
        _ if is_even(count) => sum_even % 10 == 0, // uses sum_even if count is even
        _ => sum_odd % 10 == 0,                    // uses sum_odd if count is odd
    }
}

fn is_even(n: u8) -> bool {
    n & 1 == 0
}

fn calculates(n: u8, double: bool) -> u8 {
    match (double, n > 4) {
        // if val > 4 then val * 2 > 9
        (true, true) => (n << 1) - 9, // val * 2 - 9
        (true, false) => (n << 1),    // val * 2
        _ => n,
    }
}
