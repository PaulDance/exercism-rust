use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Palindrome {
    /// Product of first decomposition received.
    value: u64,
}

impl Palindrome {
    /// Builds a `Palindrome` from the given two-factor decomposition.
    fn from_tfd(a: u64, b: u64) -> Self {
        Self { value: a * b }
    }

    pub fn new(n: u64) -> Option<Self> {
        n.is_palindrome().then(|| Self { value: n })
    }

    /// Returns the numerical value of the palindrome.
    pub fn into_inner(&self) -> u64 {
        self.value
    }
}

/// `PartialOrd` implementation for `Palindrome` based on `Palindrome::value`.
impl PartialOrd for Palindrome {
    /// Returns the partial comparison between `self`'s `value` and `other`'s.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// Provides one method: `is_palindrome`.
trait IsPalindrome {
    /// Returns `true` iff `self` is a palindrome.
    fn is_palindrome(&self) -> bool;
}

impl IsPalindrome for str {
    fn is_palindrome(&self) -> bool {
        for (i, (c1, c2)) in self.chars().zip(self.chars().rev()).enumerate() {
            if i > self.len() / 2 {
                return true;
            } else if c1 != c2 {
                return false;
            }
        }

        true
    }
}

impl IsPalindrome for u64 {
    fn is_palindrome(&self) -> bool {
        self.to_string().is_palindrome()
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut map = HashMap::new();

    // Find all the product palindromes,
    for (p, (i, j)) in (min..=max)
        .flat_map(|i| (i..=max).map(move |j| (i * j, (i, j))))
        .filter(|(p, _)| p.is_palindrome())
    {
        if !map.contains_key(&p) {
            map.insert(p, Palindrome::from_tfd(i, j));
        }
    }

    // then find the potential min and max among them.
    let min_min = &Palindrome::from_tfd(0, 0);
    let max_max = &Palindrome::from_tfd(1, u64::MAX);
    let mut min = max_max;
    let mut max = min_min;

    for p in map.values() {
        if p < min {
            min = p;
        }

        if p > max {
            max = p;
        }
    }

    if min == max_max {
        if max == min_min {
            None
        } else {
            Some((max.clone(), max.clone()))
        }
    } else if max == min_min {
        Some((min.clone(), min.clone()))
    } else {
        Some((min.clone(), max.clone()))
    }
}
