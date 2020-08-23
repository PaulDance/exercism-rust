/// Returns `true` iff `year` represents a leap year in the Gregorian calendar.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use leap::is_leap_year;
/// assert_eq!(is_leap_year(1996), true);
/// assert_eq!(is_leap_year(1997), false);
/// ```
pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
