pub fn nth(n: u32) -> u32 {
    nth_vb(n)
}

fn nth_va(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut next: u32 = 5;
    while n + 1 > primes.len() as u32 {
        match (next..)
            .step_by(2)
            .find(|x| !primes.iter().any(|&p| x % p == 0))
        {
            Some(prime) => {
                next = prime + 2;
                primes.push(prime);
            }
            None => break,
        }
    }
    primes[n as usize]
}

fn nth_vb(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut next: u32 = 5;
    while n + 1 > primes.len() as u32 {
        match (next..).step_by(2).find(|&x| is_prime(x)) {
            Some(prime) => {
                next = prime + 2;
                primes.push(prime);
            }
            None => break,
        }
    }
    primes[n as usize]
}

fn is_prime(n: u32) -> bool {
    is_prime_vb(n)
}

fn is_prime_va(n: u32) -> bool {
    match true {
        _ if n <= 3 => n > 1,
        _ if n % 2 == 0 || n % 3 == 0 => false,
        _ => {
            let mut i = 5;
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}

fn is_prime_vb(n: u32) -> bool {
    match true {
        _ if n <= 3 => n > 1,
        _ if n % 2 == 0 || n % 3 == 0 => false,
        _ => !(5..=((n as f64).sqrt() as u32))
            .step_by(6)
            .any(|i| n % i == 0 || n % (i + 2) == 0),
    }
}
