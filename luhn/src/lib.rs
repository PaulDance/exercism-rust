/// Checks a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // We can reject small strings immediately.
    if code.len() <= 1 {
        false
    } else {
        let mut sum = 0;
        let mut size = 0;

        // A loop is required in order to return early rejections.
        for (i, chr) in code
            .chars()
            .rev()
            .filter(|chr| !chr.is_whitespace())
            .enumerate()
        {
            if !chr.is_digit(10) {
                return false;
            } else {
                let digit = chr.to_digit(10).unwrap();

                // The sum is updated each digit.
                sum += if i % 2 == 1 {
                    let double = 2 * digit;

                    if double > 9 {
                        double - 9
                    } else {
                        double
                    }
                } else {
                    digit
                }
            }

            size += 1;
        }

        // The length in digits is computed late.
        if size <= 1 {
            false
        } else {
            sum % 10 == 0
        }
    }
}
