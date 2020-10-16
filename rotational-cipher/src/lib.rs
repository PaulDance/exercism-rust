/// `'A'`
const ALPHA_UPPER_START: u8 = 65;
/// `'a'`
const ALPHA_LOWER_START: u8 = 97;
const ALPHA_LENGTH: u8 = 26;

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|chr| {
            if !chr.is_ascii_alphabetic() {
                chr
            } else {
                // The operation is case-dependent.
                let alpha_start = if chr.is_ascii_lowercase() {
                    ALPHA_LOWER_START
                } else {
                    ALPHA_UPPER_START
                };
                (alpha_start
                    + (chr as i8 - alpha_start as i8 + key).rem_euclid(ALPHA_LENGTH as i8) as u8)
                    as char
            }
        })
        .collect()
}
