use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut map = HashMap::<char, u32>::new();

    // Count each lowercase character.
    for chr in candidate
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_alphabetic())
    {
        map.insert(
            chr,
            match map.get(&chr) {
                Some(&count) => count,
                None => 0,
            } + 1,
        );
    }

    // Return true iff all counts are exactly one.
    map.values().all(|&count| count == 1)
}
