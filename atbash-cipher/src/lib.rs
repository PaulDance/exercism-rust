const ALPHA_START: u8 = 97;
const ALPHA_LENGTH: u8 = 26;

/// Symmetric transformation of the Atbash cipher.
fn transformation(chr: char) -> char {
    if chr.is_numeric() {
        chr
    } else {
        (2 * ALPHA_START + ALPHA_LENGTH - chr as u8 - 1) as char
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|chr| chr.to_ascii_lowercase())
        .map(transformation)
        .intersperse_every(' ', 5) // See below.
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|chr| !chr.is_whitespace())
        .map(transformation)
        .collect()
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
