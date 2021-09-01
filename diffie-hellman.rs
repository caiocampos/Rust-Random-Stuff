use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

pub fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        0
    } else {
        let mut exp = exponent;
        let mut b = base;
        let mut res = 1;
        while exp > 0 {
            if is_odd(exp) {
                res = (res * b) % modulus;
            }
            exp >>= 1; // exp = exp / 2
            b = (b * b) % modulus;
        }
        res
    }
}

pub fn is_even(n: u64) -> bool {
    n | 1 != n // n % 2 == 0
}

pub fn is_odd(n: u64) -> bool {
    n | 1 == n // n % 2 != 0
}
