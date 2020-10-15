use num::Integer;

const ALPHA_START: u8 = 97;
const ALPHA_LENGTH: u8 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a.gcd(&(ALPHA_LENGTH as i32)) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(plaintext
            .chars()
            .filter(|chr| chr.is_alphanumeric())
            .map(|chr| chr.to_ascii_lowercase())
            .map(|chr| {
                if chr.is_numeric() {
                    chr
                } else {
                    (ALPHA_START
                        + (a * (chr as u8 - ALPHA_START) as i32 + b).rem_euclid(ALPHA_LENGTH as i32)
                            as u8) as char
                }
            })
            .intersperse_every(' ', 5) // See below.
            .collect())
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let xgcd = a.extended_gcd(&(ALPHA_LENGTH as i32));

    if xgcd.gcd != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(ciphertext
            .chars()
            .filter(|chr| !chr.is_ascii_whitespace())
            .map(|chr| {
                if chr.is_numeric() {
                    chr
                } else {
                    (ALPHA_START
                        + (xgcd.x * ((chr as u8 - ALPHA_START) as i32 - b))
                            .rem_euclid(ALPHA_LENGTH as i32) as u8) as char
                }
            })
            .collect())
    }
}

/// Small extension on iterators enabling adding a cloned element every once in a while.
trait MoreIter: Iterator {
    fn intersperse_every(self, element: Self::Item, every: usize) -> IntersperseEvery<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        IntersperseEvery::new(self, element, every)
    }
}

/// Add it to all iterators.
impl<I: ?Sized> MoreIter for I where I: Iterator {}

/// The supporting structure.
struct IntersperseEvery<I: Iterator> {
    /// A look-ahead variable from `iter.next()` for fusing and avoiding the last clone.
    peek: Option<I::Item>,
    /// The iterator to consume elements from.
    iter: I,
    /// The current yielded element's position.
    index: usize,
    /// Constant: the interspersing period.
    every: usize,
    /// Constant: the element to intersperse.
    element: I::Item,
}

impl<I> IntersperseEvery<I>
where
    I: Iterator,
    I::Item: Clone,
{
    /// "Hides" implentation details.
    fn new(mut iter: I, element: I::Item, every: usize) -> Self {
        Self {
            peek: iter.next(),
            iter,
            index: 0,
            every: every + 1, // See below.
            element,
        }
    }
}

/// The interspersing itself.
impl<I> Iterator for IntersperseEvery<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Fuse early.
        if self.peek.is_none() {
            None
        } else {
            // Increment the index every time, otherwise the iterator gets
            // stuck on a loop, hence the `every: every + 1` used above.
            self.index += 1;

            // Add element when period has passed, but not at extremities.
            if self.index % self.every == 0 {
                Some(self.element.clone())
            } else {
                let next = self.peek.take();
                self.peek = self.iter.next();
                next
            }
        }
    }
}
