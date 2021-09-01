use std::u32::MAX;

fn main() {
    println!("min {}, max {}, list {:?}", 0, 0, prime_list(0, 0));
    println!("min {}, max {}, list {:?}", 11, 5, prime_list(11, 5));
    println!("min {}, max {}, list {:?}", 11, 11, prime_list(11, 11));
    println!("min {}, max {}, list {:?}", 0, 4, prime_list(0, 4));
    println!("min {}, max {}, list {:?}", 0, 5, prime_list(0, 5));
    println!("min {}, max {}, list {:?}", 4, 10, prime_list(4, 10));
    println!("min {}, max {}, list {:?}", 5, 10, prime_list(5, 10));
    println!("min {}, max {}, list {:?}", 0, 10, prime_list(0, 10));
    println!("min {}, max {}, list {:?}", 10, 100, prime_list(10, 200));
    println!("1º {}, 4º {}, 6º {:?}, 10º {:?}, 100º {:?}", nth(0), nth(3), nth(5), nth(9), nth(99));
    if let Some(prime) = (0..=MAX).rev().find(is_prime_ref) {
        println!("MAX {}", prime);
    }
    (0..=10)
        .filter(is_prime_ref)
        .for_each(|i| println!("{}", i));
}

pub fn prime_list(begin: u32, end: u32) -> Vec<u32> {
    match (begin, end) {
        (b, e) if b == e => match is_prime(b) {
            true => vec![b],
            _ => Vec::new(),
        },
        (b, e) if b > e => Vec::new(),
        (b, e) => (b..=e)
            .filter(|&i| i == 2 || is_odd(i))
            .filter(is_prime_ref)
            .collect(),
    }
}

fn nth(n: u32) -> u32 {
    let mut primes = vec![2u32];
    let mut next = 3u32;
	let res = (1..=n).try_for_each(|_| {
        match (next..).step_by(2).find(is_prime_ref) {
            Some(prime) => {
                next = prime + 2;
                primes.push(prime);
				Ok(())
            }
            None => Err(()),
        }
	});
	if res.is_err() {
		return 0;
	}
    primes[n as usize]
}

pub fn is_prime(num: u32) -> bool {
    is_prime_ref(&num)
}

pub fn is_prime_ref(num: &u32) -> bool {
    match num {
        0 | 1 => false,
		2 | 3 => true,
        &n if is_even(n) || n % 3 == 0 => false,
        &n => !(5..=((n as f64).sqrt() as u32))
            .step_by(6)
            .any(|i| n % i == 0 || n % (i + 2) == 0),
    }
}

pub fn is_even(num: u32) -> bool {
    (num & 1) == 0
}

pub fn is_odd(num: u32) -> bool {
    (num & 1) == 1
}
