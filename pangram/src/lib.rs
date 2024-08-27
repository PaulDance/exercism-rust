use std::collections::HashMap;
use unidecode::unidecode_char; // To remove accents.

/// Returns a HashMap where keys are all the characters 'a' to 'z' and values are their counts.
fn count_alphabet(input: &str) -> HashMap<char, usize> {
    let mut map: HashMap<_, _> = ('a'..='z').map(|chr| (chr, 0)).collect();

    input
        .chars()
        .map(|chr| {
            unidecode_char(chr.to_ascii_lowercase())
                .chars()
                .next()
                .unwrap()
        })
        .filter(|chr| chr.is_alphabetic())
        .for_each(|chr| {
            *map.entry(chr).or_default() += 1;
        });

    map
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    !sentence.is_empty() && count_alphabet(sentence).values().all(|&n| n > 0)
}
