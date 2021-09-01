/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let res = code
        .chars()
        .filter(|&el| el != ' ')
        .rev()
        .enumerate()
        .map(process)
        .fold((true, 0usize, 0u32), accumulate);
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

type Result = (bool, usize, u32);

fn process((i, el): (usize, char)) -> Result {
    if let Some(val) = el.to_digit(10) {
        (true, i + 1, calculates(val, i % 2 == 1))
    } else {
        (false, 0, 0)
    }
}

fn accumulate((status, _, sum): Result, (ok, i, el): Result) -> Result {
    (status && ok, i, sum + el)
}
