extern crate rayon;

use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match worker_count.min(input.len()) {
        1 => frequency_seq(input),
        n => (0..n)
            .into_par_iter()
            .map(|i| {
                let v: Vec<&str> = input.iter().skip(i).step_by(n).cloned().collect();
                frequency_seq(&v)
            })
            .reduce_with(merge)
            .unwrap_or(HashMap::new()),
    }
}

fn frequency_seq(input: &[&str]) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|&s| s.chars())
        .filter(|&c| c.is_alphabetic())
        .fold(HashMap::new(), |acc, c| {
            update_freq(acc, c.to_ascii_lowercase(), 1)
        })
}

fn merge(map: HashMap<char, usize>, other: HashMap<char, usize>) -> HashMap<char, usize> {
    other
        .iter()
        .fold(map, |acc, (&key, &val)| update_freq(acc, key, val))
}

fn update_freq(mut map: HashMap<char, usize>, key: char, val: usize) -> HashMap<char, usize> {
    *map.entry(key).or_insert(0) += val;
    map
}
