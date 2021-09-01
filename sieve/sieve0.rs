pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    match upper_bound {
        0 | 1 => Vec::new(),
        2 => vec![2],
        _ => {
            let upper = upper_bound as usize;
            let (mut list, max) = (vec![true; upper + 1], upper / 2);
            (2..=max).for_each(|i| {
                (i..=max).for_each(|j| {
                    if let Some(el) = list.get_mut(i * j) {
                        *el = false;
                    }
                })
            });
            (2..=upper)
                .filter_map(|i| match list[i] {
                    true => Some(i as u64),
                    _ => None,
                })
                .collect()
        }
    }
}
