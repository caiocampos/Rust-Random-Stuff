pub fn factors(n: u64) -> Vec<u64> {
    let mut val: u64 = n;
    let mut min: u64 = 2;
    let mut res: Vec<u64> = Vec::new();
    loop {
        match (min..=val).find(|x| val % x == 0) {
            Some(prime) => {
                val /= prime;
                min = prime;
                res.push(prime);
            },
            None => break
        }
    }
    res
}
