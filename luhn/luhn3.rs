/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (mut count, mut sum) = (0, 0);
    let valid = code.chars().rev().all(|el| match el {
        '0'..='9' => {
            let val = match el.to_digit(10) {
                Some(n) => n,
                _ => return false,
            };
            sum += calculates(val, count % 2 == 1);
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

fn calculates(n: u32, double: bool) -> u32 {
    match (double, n > 4) {
        (true, true) => n * 2 - 9, // if val > 4 then val * 2 > 9
        (true, false) => n * 2,
        _ => n,
    }
}
