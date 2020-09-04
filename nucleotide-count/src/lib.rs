use std::collections::HashMap;
use std::iter::FromIterator;
const NUCLEOTIDES: &'static str = "ATCG";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(nucleotide) {
        Err(nucleotide)
    } else {
        let mut count = 0;

        for chr in dna.chars() {
            if !NUCLEOTIDES.contains(chr) {
                return Err(chr);
            } else if chr == nucleotide {
                count += 1;
            }
        }

        Ok(count)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::from_iter(NUCLEOTIDES.chars().map(|chr| (chr, 0)));

    for chr in dna.chars() {
        if !NUCLEOTIDES.contains(chr) {
            return Err(chr);
        } else {
            map.insert(chr, map.get(&chr).unwrap() + 1);
        }
    }

    Ok(map)
}
