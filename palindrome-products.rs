#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    value: u64,
    list: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            list: vec![Self::factors(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        if self.value != a * b {
            return;
        }
        let factors = Self::factors(a, b);
        if !self.list.contains(&factors) {
            self.list.push(factors);
        }
    }

    pub fn validate(value: u64) -> bool {
        match value {
            1..=9 => true,
            p if p % 10 == 0 => false,
            p @ 10..=99 => p % 11 == 0,
            p => Self::validate_loop(p),
        }
    }

    #[allow(dead_code)]
    fn validate_loop(value: u64) -> bool {
        let (mut n, mut rev) = (value, 0);
        while n > 0 {
            rev = rev * 10 + n % 10;
            n /= 10;
        }
        value == rev
    }

    #[allow(dead_code)]
    fn validate_iter(value: u64) -> bool {
        let digits = value.to_string();
        digits.chars().rev().eq(digits.chars())
    }

    fn factors(a: u64, b: u64) -> (u64, u64) {
        match a > b {
            true => (b, a),
            _ => (a, b),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PalindromePair {
    Pair(Palindrome, Palindrome),
    Single(Palindrome),
    None,
}

impl PalindromePair {
    pub fn add(self, a: u64, b: u64) -> Self {
        if !Palindrome::validate(a * b) {
            return self;
        }
        match self {
            PalindromePair::Pair(mut min, mut max) => match a * b {
                p if p == min.value() => {
                    min.insert(a, b);
                    PalindromePair::Pair(min, max)
                }
                p if p == max.value() => {
                    max.insert(a, b);
                    PalindromePair::Pair(min, max)
                }
                p if p > max.value() => PalindromePair::Pair(min, Palindrome::new(a, b)),
                p if p < min.value() => PalindromePair::Pair(Palindrome::new(a, b), max),
                _ => PalindromePair::Pair(min, max),
            },
            PalindromePair::Single(mut el) => match a * b {
                p if p == el.value() => {
                    el.insert(a, b);
                    PalindromePair::Single(el)
                }
                p if p > el.value() => PalindromePair::Pair(el, Palindrome::new(a, b)),
                p if p < el.value() => PalindromePair::Pair(Palindrome::new(a, b), el),
                _ => PalindromePair::Single(el),
            },
            PalindromePair::None => PalindromePair::Single(Palindrome::new(a, b)),
        }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let pair = (min..=max).fold(PalindromePair::None, |pair, i| {
        (i..=max).fold(pair, |p, j| p.add(i, j))
    });
    match pair {
        PalindromePair::Pair(el_min, el_max) => Some((el_min, el_max)),
        PalindromePair::Single(el) => Some((el.clone(), el)),
        PalindromePair::None => None,
    }
}
