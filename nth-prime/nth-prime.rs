use rayon::prelude::*;

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![]; 
    let mut next: u32 = 2;
    while n + 1 > primes.len() as u32 {
        // match (next..).find(|x| {
        match (next..std::u32::MAX).into_par_iter().find_first(|x| {
            // !primes.iter().any(|&p| x % p == 0)
            !primes.par_iter().any(|&p| x % p == 0)
        }) {
            Some(prime) => ({
                next = prime + 1;
                primes.push(prime);
            }),
            None => break
        }
    }
    primes[n as usize]
}

pub fn is_prime(n: u32) -> bool {
    !(2..n).any(|i| n % i == 0)
}

pub fn nth2(n: u32) -> u32 {
    (2..).filter(|c| is_prime(*c)).nth(n as usize).unwrap()
}