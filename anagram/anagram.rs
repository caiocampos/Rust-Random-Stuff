use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_counter: HashMap<char, u8> = HashMap::new();
    let word = word.to_lowercase();
    word.chars().for_each(|c| {
        word_counter.entry(c).and_modify(|f| *f += 1).or_insert(1);
    });
    let mut candidate_counter: HashMap<char, u8> = HashMap::new();
    possible_anagrams
        .iter()
        .cloned()
        .filter_map(|candidate| {
            let el = candidate.to_lowercase();
            if el.len() != word.len() || word == el {
                return None;
            }
            candidate_counter.clear();
            for c in el.chars() {
                if !word_counter.contains_key(&c) {
                    return None;
                }
                let freq = candidate_counter
                    .entry(c)
                    .and_modify(|f| *f += 1)
                    .or_insert(1);
                if *freq > word_counter[&c] {
                    return None;
                }
            }
            Some(candidate)
        })
        .collect()
}
