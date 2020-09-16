/// Vowel rules.
const VOWELS: [&str; 7] = ["a", "o", "e", "i", "u", "xr", "yt"];
/// Consonant rules.
const CONSONANTS: [(usize, &str); 9] = [
    (0, "ch"),
    (1, "qu"),
    (0, "qu"),
    (0, "sch"),
    (0, "rh"),
    (0, "thr"),
    (0, "th"),
    (0, "xr"),
    (0, "yt"),
];

/// Partially converts the given `word` following vowel rules only.
fn convert_vowels(word: &str) -> Option<String> {
    VOWELS
        .iter()
        .find(|&&vow| word.starts_with(vow))
        .map(|_| word.to_string() + "ay")
}

/// Partially converts the given `word` following consonant rules only.
fn convert_consonants(word: &str) -> String {
    CONSONANTS
        .iter()
        .find(|&(i, cons)| word[*i..].starts_with(cons))
        .map_or(word[1..].to_string() + &word[0..1] + "ay", |&(i, cons)| {
            let cut = cons.len() + i;
            word[cut..].to_string() + &word[..cut] + "ay"
        })
}

/// Fully converts the given `word` to Pig Latin.
fn convert_word(word: &str) -> String {
    convert_vowels(word).unwrap_or_else(|| convert_consonants(word))
}

/// Translates the given `sentence` to Pig Latin.
pub fn translate(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| convert_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}
