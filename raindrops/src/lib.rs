/// Converts the given number into raindrops as a `String`.
///
/// `raindrops` builds a `String` following these rules in order: if the given
/// number
///
///  * has 3 as a factor, adds 'Pling' to the result.
///  * has 5 as a factor, adds 'Plang' to the result.
///  * has 7 as a factor, adds 'Plong' to the result.
///  * *does not* have any of 3, 5, or 7 as a factor, the result is the digits
///    of the number.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use raindrops::raindrops;
/// assert_eq!(raindrops(28), "Plong");
/// assert_eq!(raindrops(30), "PlingPlang");
/// assert_eq!(raindrops(34), "34");
/// ```

pub fn raindrops(n: u32) -> String {
    let mut ppp = String::with_capacity(15);

    if n % 3 == 0 {
        ppp.extend("Pling".chars());
    }

    if n % 5 == 0 {
        ppp.extend("Plang".chars());
    }

    if n % 7 == 0 {
        ppp.extend("Plong".chars());
    }

    if !ppp.is_empty() {
        ppp
    } else {
        n.to_string()
    }
}
