/// Computes the zero-based n-th prime number and returns it.
///
/// Uses lazy iterators in order to do so.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use nth_prime::nth;
/// assert_eq!(nth(0), 2);
/// assert_eq!(nth(8), 23);
/// assert_eq!(nth(24), 97);
/// ```
pub fn nth(n: usize) -> usize {
    (2..)
        .filter(|&p| (2..p).all(|q| p % q != 0))
        .nth(n)
        .unwrap()
}
