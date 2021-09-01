pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut steps = 0;
            let mut n = n;

            while n != 1 {
                n = if is_odd(n) {
                    3 * n + 1
                } else {
                    n >> 1 // n / 2
                };
                steps += 1;
            }
            Some(steps)
        }
    }
}

pub fn is_even(n: u64) -> bool {
    n | 1 != n // n % 2 == 0
}

pub fn is_odd(n: u64) -> bool {
    n | 1 == n // n % 2 != 0
}
