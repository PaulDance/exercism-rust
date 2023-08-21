use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};

use num::bigint::BigInt;
use num::traits::Pow;

/// Type implementing arbitrary-precision decimal arithmetic.
/// Representation: mant * 10^-exp
#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Decimal {
    /// Exponent of the decimal in radix 10.
    exp: usize,
    /// Mantissa of the decimal in radix 10.
    mant: BigInt,
}

impl Decimal {
    /// Parses a decimal from a given string slice or returns `None` if it
    /// is invalid.
    pub fn try_from(input: &str) -> Option<Self> {
        // Parts before and after the decimal separator.
        let parts = input.split('.').collect::<Vec<_>>();

        if parts.len() > 2 {
            None
        } else {
            // Clean parts.
            let left = parts[0];
            let right = if parts.len() == 2 {
                parts[1].trim_end_matches('0')
            } else {
                ""
            };

            // Produce the value using bytes length and content.
            Some(Self {
                exp: right.len(),
                mant: BigInt::parse_bytes((left.to_string() + right).as_bytes(), 10).unwrap(),
            })
        }
    }

    /// Returns the `(min, max)` couple of decimals where `min` has the
    /// greatest exponent of the two, thus representing a smaller value
    /// when only considering exponents.
    fn min_max_by_exp(self, other: Self) -> (Self, Self) {
        if self.exp >= other.exp {
            (self, other)
        } else {
            (other, self)
        }
    }

    /// Returns the normalized version of the decimal, i.e. with minimal
    /// exponent by removing trailing zeroes.
    fn normalize(self) -> Self {
        let repr = self.mant.to_string();
        let zeroes = repr.len() - repr.trim_end_matches('0').len();
        Self {
            exp: self.exp - zeroes,
            mant: self.mant / BigInt::from(10).pow(zeroes),
        }
    }
}

//     e1 >= e2
// =>  m1 * 10^-e1 + m2 * 10^-e2
//  =  10^-e1 * (m1 + m2 * 10^(e1-e2))
impl Add<Self> for Decimal {
    type Output = Self;

    /// Adds the two decimals together into a new one.
    fn add(self, other: Self) -> Self::Output {
        let (min, max) = self.min_max_by_exp(other);
        Self {
            exp: min.exp,
            mant: min.mant + max.mant * BigInt::from(10).pow(min.exp - max.exp),
        }
        .normalize()
    }
}

//   -(m * 10^-e)
// = -m * 10^-e
impl Neg for Decimal {
    type Output = Self;

    /// Inverts the mantissa of the decimal.
    fn neg(mut self) -> Self::Output {
        self.mant = -self.mant;
        self
    }
}

//   x - y
// = x + -y
impl Sub<Self> for Decimal {
    type Output = Self;

    /// Subtracts `other` from `self` using a negation and an addition.
    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

//   (m1 * 10^-e1) * (m2 * 10^-e2)
// = (m1 * m2) * 10^-(m1 + m2)
impl Mul<Self> for Decimal {
    type Output = Self;

    /// Multiplies the two decimals by multiplying the mantissae and adding
    /// the exponents.
    fn mul(self, other: Self) -> Self::Output {
        Self {
            exp: self.exp + other.exp,
            mant: self.mant * other.mant,
        }
        .normalize()
    }
}

//      m1 * 10^-e1 <= m2 * 10^-e2
// <=>  m1 * 10^e2 <= m2 * 10^e1
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.mant.sign() != other.mant.sign() {
            self.mant.sign().partial_cmp(&other.mant.sign())
        } else if self.exp == other.exp {
            self.mant.partial_cmp(&other.mant)
        } else {
            (&self.mant * BigInt::from(10).pow(other.exp))
                .partial_cmp(&(&other.mant * BigInt::from(10).pow(self.exp)))
        }
    }
}
