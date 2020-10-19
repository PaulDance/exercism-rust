use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    // Split by punctuation or whitespace but leave quotes in words.
    for word in words
        .split(|chr: char| chr != '\'' && (chr.is_ascii_whitespace() || chr.is_ascii_punctuation()))
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches(|chr: char| chr.is_ascii_punctuation()))
        .map(str::to_ascii_lowercase)
    {
        *map.entry(word).or_insert(0) += 1;
    }

    map
}
