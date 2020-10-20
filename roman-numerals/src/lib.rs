use std::fmt::{Display, Formatter, Result};

/// Roman numerals are in limited number, so writing them out is simpler.
const NUMERALS: [[&str; 9]; 4] = [
    ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ["X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ["M", "MM", "MMM", "M?", "?", "?M", "?MM", "?MMM", "M?"],
];

/// Represents a Roman numeral.
pub struct Roman {
    /// The array of radix 10 digits composing the number.
    digits: Vec<usize>,
}

/// Positions index in `NUMERALS` and digits index in `NUMERALS`'s arrays.
impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(
            &self
                .digits
                .iter()
                .enumerate()
                .filter(|&(_, &digit)| digit != 0)
                .map(|(i, digit)| NUMERALS[self.digits.len() - i - 1][digit - 1])
                .collect::<String>(),
        )
    }
}

/// Split the number in digits.
impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self {
            digits: num
                .to_string()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap() as usize)
                .collect(),
        }
    }
}
