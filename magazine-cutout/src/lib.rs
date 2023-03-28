// In order to get its `counts` function.
use itertools::Itertools;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let word_counts = magazine.into_iter().copied().counts();
    note.into_iter()
        .copied()
        .counts()
        .into_iter()
        .all(|(w, c)| word_counts.get(w).copied().unwrap_or(0) >= c)
}
