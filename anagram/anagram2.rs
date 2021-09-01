use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let el = candidate.to_lowercase();
            if el.len() != word.len() || word == el {
                return false;
            }
            word.chars().all(|c| test(c, &word, &el))
        })
        .cloned()
        .collect()
}

fn test(c: char, word: &String, candidate: &String) -> bool {
    word.matches(c).count() == candidate.matches(c).count()
}
