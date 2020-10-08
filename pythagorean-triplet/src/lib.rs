//! Most of the properties used here are taken from:
//! https://en.wikipedia.org/wiki/Pythagorean_triple

use std::collections::HashSet;

/// Given the sum {}, returns all possible Pythagorean triples which produce
/// the said sum, or an empty HashSet if there are no such triples. Note that
/// it returns triples in [a, b, c] order, where a < b < c.
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();

    // Using Euclid's formula, we know the sum must be even for a triplet to exist.
    if sum % 2 == 0 {
        find_by_tree(3, 4, 5, sum, &mut set);
    }

    set
}

/// Recursive function that follows the tree of primitive Pythagorean triples
/// in order to fill the given `set` with triples which sum is `n`. `a`, `b`
/// and `c` must form a valid primitive triple, preferably the root (3, 4, 5).
fn find_by_tree(a: u32, b: u32, c: u32, n: u32, set: &mut HashSet<[u32; 3]>) {
    // Re-order a and b: useful for recursion.
    let (a, b) = (a.min(b), a.max(b));
    let sum = a + b + c;

    // Using the derivation formulas and that a < b < c, one can easily prove
    // that the child primitive triples have greater elements than their parent
    // when comparing by column, therefore the sum too. We thus may stop searching
    // as soon as the sum is greater than N.
    if sum <= n {
        // Any multiple of a triple is also a triple: we thus know we are able
        // to generate all triples because the current one is primitive.
        for k in (1..=n / sum).filter(|k| k * sum == n) {
            set.insert([k * a, k * b, k * c]);
        }

        // Recursive calls by derivating the current triple in order to generate
        // the subsequent branches of the tree. Formulas taken from Wikipedia.
        find_by_tree(
            a + 2 * (c - b),
            2 * (a + c) - b,
            2 * a + 3 * c - 2 * b, // Avoid underflow here.
            n,
            set,
        );
        find_by_tree(
            a + 2 * (b + c),
            2 * (a + c) + b,
            2 * (a + b) + 3 * c,
            n,
            set,
        );
        find_by_tree(
            2 * (b + c) - a,
            2 * (c - a) + b,
            2 * (b - a) + 3 * c,
            n,
            set,
        );
    }
}
