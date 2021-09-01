use rayon::prelude::*;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    find_vb(sum)
}

pub struct DivisionResult {
    quotient: u32,
    remainder: u32,
}

pub fn exact_div(dividend: u32, divisor: u32) -> DivisionResult {
    let mut res = DivisionResult {
        quotient: 0,
        remainder: dividend % divisor,
    };
    if res.remainder != 0 {
        res
    } else {
        res.quotient = dividend / divisor;
        res
    }
}

// last version
pub fn find_vb(sum: u32) -> HashSet<[u32; 3]> {
    // a < b < c
    // 1 < 2 < 3 - nothing smaller is possíble
    // min sum == 6
    if sum < 6 {
        return HashSet::new();
    }
    let max = sum / 3;
    (1..max)
        .into_par_iter()
        .filter_map(|a| {
            // a.pow(2) + b.pow(2) == c.pow(2)
            // b.pow(2) == c.pow(2) - a.pow(2)
            // (2*b*c + b.pow(2)) + b.pow(2) == c.pow(2) - a.pow(2) + (2*b*c + b.pow(2))
            // 2*b*c + 2*b.pow(2) == c.pow(2) + 2*b*c + b.pow(2) - a.pow(2)
            // 2*b*(c + b) == (c + b).pow(2) - a.pow(2)
            // 2*b*(c + b) == ((c + b) + a) * ((c + b) - a)
            // 2*b*(c + b) == (c + b + a) * (c + b - a)
            // b == (c + b + a) * (c + b - a) / 2*(c + b)
            // ...
            // c + b + a == sum
            // c + b + a - (2*a) == sum - (2*a)
            // c + b - a == sum - 2*a
            // ...
            // c + b + a == sum
            // c + b == sum - a
            // ...
            // b == sum * (sum - 2*a) / 2*(sum - a)
            let sma = sum - a;
            let bd = exact_div(sum * (sma - a), 2 * sma);
            if bd.remainder != 0 {
                return None;
            }
            let b = bd.quotient;
            // c + b = sum - a
            // c = sum - a - b
            if a >= b || b >= sma {
                return None;
            }
            let c = sma - b;
            if b >= c {
                return None;
            }
            // a.pow(2) + b.pow(2) == c.pow(2)
            // a.pow(2) == c.pow(2) - b.pow(2)
            // a.pow(2) == (c - b) * (c + b)
            // a == (c - b) * (c + b) / a
            // ...
            // c + b == sum - a
            let ad = exact_div((c - b) * sma, a);
            if (ad.remainder == 0) && (a == ad.quotient) {
                Some([a, b, c])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}

// original version
pub fn find_va(sum: u32) -> HashSet<[u32; 3]> {
    // a < b < c
    // 1 < 2 < 3 - nothing smaller is possíble
    // min sum == 6
    if sum < 6 {
        return HashSet::new();
    }
    let max = sum / 3;
    (1..max)
        .into_par_iter()
        .filter_map(|a| {
            let a2 = a * a;
            // a2 + b.pow(2) == c.pow(2)
            // a2 == c.pow(2) - b.pow(2)
            // a2 == (c - b) * (c + b)
            // ...
            // a + b + c == sum
            // b + c == sum - a
            // ...
            // a2 == (c - b) * (sum - a)
            let sma = sum - a;
            // a2 == (c - b) * sma
            // c - b == a2 / sma
            if a2 < sma {
                return None;
            }
            let dif = exact_div(a2, sma);
            if dif.remainder != 0 {
                return None;
            }
            // dif == c - b
            let dif = dif.quotient;
            // a + b + c == sum
            // ...
            // c == b + dif
            // ...
            // a + b + b + dif == sum
            // 2 * b == sum - a - dif
            // b == (sum - a - dif) / 2;
            let b = (sma - dif) / 2;
            // a < b < c
            if a >= b {
                return None;
            }
            let c = b + dif;
            if sum == a + b + c {
                Some([a, b, c])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
