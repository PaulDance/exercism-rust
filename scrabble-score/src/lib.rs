use lazy_static::lazy_static;
use maplit::hashmap;

use std::collections::HashMap;

lazy_static! {
    /// The mapping between English Scrabble letters and their scores.
    static ref SCORE: HashMap<char, u64> = hashmap! {
        'A' => 1, 'E' => 1, 'I' => 1, 'O' => 1, 'U' => 1,
        'L' => 1, 'N' => 1, 'R' => 1, 'S' => 1, 'T' => 1,
        'D' => 2, 'G' => 2,
        'B' => 3, 'C' => 3, 'M' => 3, 'P' => 3,
        'F' => 4, 'H' => 4, 'V' => 4, 'W' => 4, 'Y' => 4,
        'K' => 5,
        'J' => 8, 'X' => 8,
        'Q' => 10, 'Z' => 10,
    };
}

/// Computes the simple Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .filter_map(|chr| SCORE.get(&chr.to_ascii_uppercase()))
        .sum()
}
