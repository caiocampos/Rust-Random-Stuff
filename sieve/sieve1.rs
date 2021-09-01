pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut candidates: Vec<_> = (2..limit+1).rev().collect();
    while let Some(prime) = candidates.pop() {
        primes.push(prime);
        candidates.retain(|n| n % prime != 0);
    }
    primes
}