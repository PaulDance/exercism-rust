#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

/// Simple solution directly using suggested methods, the string is scanned twice.
pub fn lsp(digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        Ok(1)
    } else if span > digits.len() {
        Err(Error::SpanTooLong)
    } else {
        // Reject non-digit characters using an iterator.
        match digits.chars().find(|chr| !chr.is_ascii_digit()) {
            Some(chr) => Err(Error::InvalidDigit(chr)),
            None => Ok(digits
                .as_bytes()
                .windows(span)
                .map(|series| {
                    series
                        .iter()
                        .map(|&byte| (byte as char).to_digit(10).unwrap() as u64)
                        .product()
                })
                .max()
                .unwrap()),
        }
    }
}
