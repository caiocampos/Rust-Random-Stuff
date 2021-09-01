const CHESS_MIN: u32 = 1;
const CHESS_MAX: u32 = 64;
const PANIC_MESSAGE: &str = "Square must be between 1 and 64";

pub fn square(s: u32) -> u64 {
    square_vb(s)
}

pub fn pow_two(p: u32) -> u128 {
    1_u128 << p // 2_u128.pow(p)
}

pub fn mul_pow_two<T: std::ops::Shl>(n: T, p: T) -> T::Output {
    n << p // n * 2_u128.pow(p)
}

pub fn square_va(s: u32) -> u64 {
    if s < CHESS_MIN || s > CHESS_MAX {
        panic!(PANIC_MESSAGE);
    }
    pow_two(s - 1) as u64
}

pub fn square_vb(s: u32) -> u64 {
    match s {
        CHESS_MIN..=CHESS_MAX => pow_two(s - 1) as u64,
        _ => panic!(PANIC_MESSAGE),
    }
}

pub fn total() -> u64 {
    // (CHESS_MIN..=CHESS_MAX).map(|n| square(n)).sum()
    // (2_u128.pow(CHESS_MAX) - 1) as u64
    (pow_two(CHESS_MAX) - 1) as u64
}
