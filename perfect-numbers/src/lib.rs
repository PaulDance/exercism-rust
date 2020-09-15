use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

/// Conversion implementation from `aliquot_sum.cmp(&num)` to `Classification`.
impl From<Ordering> for Classification {
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Equal => Self::Perfect,
            Ordering::Greater => Self::Abundant,
            Ordering::Less => Self::Deficient,
        }
    }
}

/// Classifies the given number as per Nicomachus' scheme.
pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        _ => Some(
            (1..num)
                .filter(|p| num % p == 0)
                .sum::<u64>()
                .cmp(&num)
                .into(),
        ),
    }
}
