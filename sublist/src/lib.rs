#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Determines the `Comparison` between the given two slices.
pub fn sublist<T: PartialEq>(needle: &[T], haystack: &[T]) -> Comparison {
    if needle.is_empty() && haystack.is_empty() {   // Empty are equal;
        Comparison::Equal
    } else if needle.len() > haystack.len() {       // when sizes are unordered,
        match sublist(haystack, needle) {           // swap and call again.
            Comparison::Sublist => Comparison::Superlist,
            cmp => cmp,
        }
    } else if needle.is_empty() {
        Comparison::Sublist
    } else {
        match haystack.windows(needle.len()).find(|&sub| sub == needle) {
            Some(sub) => {
                if sub.len() == haystack.len() {
                    Comparison::Equal
                } else {
                    Comparison::Sublist
                }
            }
            _ => Comparison::Unequal,
        }
    }
}
