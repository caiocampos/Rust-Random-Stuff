const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len() < 26 {
        return false;
    }
    let sentence = sentence.to_uppercase();
    !ALPHABET.chars().any(|c| !sentence.contains(c))
}
