/// The Luhn struct: in order to re-use the previous implementation, only
/// stores the input code as `String`.
pub struct Luhn {
    code: String,
}

/// Implementation directly taken from the previous exercise.
impl Luhn {
    pub fn is_valid(&self) -> bool {
        // We can reject small strings immediately.
        if self.code.len() <= 1 {
            false
        } else {
            let mut sum = 0;
            let mut size = 0;

            // A loop is required in order to return early rejections.
            for (i, chr) in self
                .code
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
}

/// Enable the validation for anything that can be turned to a String.
impl<T: ToString> From<T> for Luhn {
    fn from(value: T) -> Self {
        Self {
            code: value.to_string(),
        }
    }
}
