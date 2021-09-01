use std::collections::HashMap;

const COUNTS: [(char, usize); 4] = [('G', 0), ('C', 0), ('T', 0), ('A', 0)];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) {
        return Err(nucleotide);
    }
    let mut count = 0;
    for c in dna.chars() {
        if !is_valid(c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = COUNTS.iter().cloned().collect();
    for c in dna.chars() {
        if !is_valid(c) {
            return Err(c);
        }
        if let Some(x) = counts.get_mut(&c) {
            *x += 1
        }
    }
    Ok(counts)
}

fn is_valid(nucleotide: char) -> bool {
    match nucleotide {
        'G' | 'C' | 'T' | 'A' => true,
        _ => false,
    }
}
