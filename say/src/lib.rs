const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TYS: [&str; 10] = [
    "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety", "hundred",
];
const ORDERS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

/// Encodes a number less than a thousand, except for zero.
///
/// Panics if it is greater than a thousand.
fn encode_thousand(n: usize) -> String {
    // Zero cannot occur here.
    if n == 0 {
        String::new()
    } else if n < 10 {
        DIGITS[n].to_string()
    } else if n < 20 {
        TEENS[n - 10].to_string()
    } else if n < 100 {
        match n % 10 {
            0 => TYS[n / 10 - 1].to_string(),
            m => format!("{}-{}", TYS[n / 10 - 1], DIGITS[m]),
        }
    } else {
        format!(
            "{} {} {}",
            DIGITS[n / 100],
            TYS[9],
            encode_thousand(n % 100)
        )
    }
}

/// Encodes any given number to US english.
pub fn encode(n: u64) -> String {
    // "Zero" may appear in the result iff n is 0.
    if n == 0 {
        DIGITS[0].to_string()
    } else {
        let mut number = String::new();
        let mut order = 6usize;
        let mut metric = 1000usize.pow(order as u32);
        let mut rest = n as usize;

        // Scan from quintillions to thousands.
        while metric != 0 {
            let magnitude = rest / metric;

            if magnitude != 0 {
                number.push_str(&format!(
                    "{} {} ",
                    encode_thousand(magnitude),
                    ORDERS[order],
                ));
            }

            rest %= metric;
            metric /= 1000;
            order = order.saturating_sub(1);
        }

        // Remove trailing whitespace but avoid using `number.trim_end().to_string()`.
        while let Some(chr) = number.pop() {
            if !chr.is_whitespace() {
                number.push(chr);
                break;
            }
        }

        number
    }
}
