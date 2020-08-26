use std::collections::{HashMap, HashSet};

/// Determines the number of occurences of the characters composing the given string slice.
///
/// Returns a `HashMap<char, u32>`: keys are the string's characters, values are their counts.
pub fn chars_freqs(word: &str) -> HashMap<char, u32> {
    let mut freqs = HashMap::new();

    for chr in word.chars() {
        freqs.insert(
            chr,
            match freqs.get(&chr) {
                Some(n) => n + 1,
                None => 1,
            },
        );
    }

    freqs
}

/// Returns `true` iff `candidate` is an non-equal anagram of `word`.
///
/// The order of arguments does not actually matter. They are not put into lower case in this
/// function however.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use anagram::is_ne_anagram;
/// assert!(is_ne_anagram("pear", "reap"));
/// assert!(! is_ne_anagram("bob", "bob"));
/// assert!(! is_ne_anagram("hEarT", "eARth"));
/// ```
pub fn is_ne_anagram(word: &str, candidate: &str) -> bool {
    word != candidate && chars_freqs(word) == chars_freqs(candidate)
}

/// Filters the given `possible_anagrams` by `is_ne_anagram` with lowercase words.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lc_word = word.to_lowercase();
    possible_anagrams
        .iter()
        .filter(|candidate| is_ne_anagram(&lc_word, &candidate.to_lowercase()))
        .map(|anagram| *anagram)
        .collect()
}
