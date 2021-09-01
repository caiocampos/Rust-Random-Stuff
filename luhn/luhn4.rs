/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let res = code
        .chars()
        .filter(|&el| el != ' ')
        .rev()
        .enumerate()
        .map(|(i, el)| {
            let val = match el.to_digit(10) {
                Some(n) => n,
                _ => return (false, 0, 0),
            };
            (true, i + 1, calculates(val, i % 2 == 1))
        })
        .fold((true, 0, 0), |(status, _, sum), (ok, i, el)| {
            (status && ok, i, sum + el)
        });
    match res {
        (true, size, sum) if size > 1 => sum % 10 == 0,
        _ => false,
    }
}

fn calculates(n: u32, double: bool) -> u32 {
    match (double, n > 4) {
        (true, true) => n * 2 - 9, // if val > 4 then val * 2 > 9
        (true, false) => n * 2,
        _ => n,
    }
}
