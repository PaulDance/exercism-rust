/// Reverses the given string slice into a `String`.
///
/// It uses the `input`'s `chars` iterator in order to manipulate UTF-8 graphemes
/// and perform the reversal properly.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use reverse_string::reverse;
/// assert_eq!(reverse("Hello, World!!!"), "!!!dlroW ,olleH".to_string());
/// ```
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
