#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

/// Converts a number between two bases.
///
/// Returns an `Err(Error::.)` if the conversion is impossible.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use allyourbase::convert;
/// assert_eq!(convert(&[4, 2], 10, 2), Ok(vec![1, 0, 1, 0, 1, 0]));
/// ```
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // Treat errors early and eagerly.
    if (0..=1).contains(&from_base) {
        Err(Error::InvalidInputBase)
    } else if (0..=1).contains(&to_base) {
        Err(Error::InvalidOutputBase)
    } else if let Some(&d) = number.iter().filter(|&&d| d >= from_base).next() {
        Err(Error::InvalidDigit(d))
    } else {
        Ok(to_radix(to_decimal(number, from_base), to_base))
    }
}

/// Converts the `number` in the given `base` into a decimal integer.
fn to_decimal(number: &[u32], base: u32) -> u128 {
    number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| d as u128 * base.pow(i as u32) as u128)
        .sum()
}

/// Converts the integer decimal `number` into digits of the given `base`.
fn to_radix(number: u128, base: u32) -> Vec<u32> {
    if number == 0 {
        vec![0]
    } else {
        let base = base as u128;
        let mut digits = Vec::new();
        let mut rest = number;

        while rest != 0 {
            digits.push((rest % base) as u32);
            rest /= base;
        }

        digits.reverse();
        digits
    }
}
