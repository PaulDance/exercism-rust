#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Converts a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.into_iter().flat_map(|&n| convert_one(n)).collect()
}

/// Converts one number to a VLQ-encoded bytes sequence.
fn convert_one(n: u32) -> Vec<u8> {
    if n == 0 {
        vec![0]
    } else {
        let mut vlq = Vec::new();
        let mut rest = n;

        // VLQ is basically base 128 decomposition.
        while rest != 0 {
            vlq.push((rest % 128) as u8 | 0x80);
            rest /= 128;
        }

        // Remove MSB from first, last after reverse.
        vlq[0] -= 0x80;
        vlq.reverse();
        vlq
    }
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes[bytes.len() - 1] & 0x80 != 0 {
        Err(Error::IncompleteNumber)
    } else {
        let mut acc = 0u32;

        bytes
            .iter()
            .filter_map(|b| match acc.overflowing_mul(128) {
                (_, true) => Some(Err(Error::Overflow)),
                (next_byte, _) => match next_byte.overflowing_add((b & 0x7f) as u32) {
                    (_, true) => Some(Err(Error::Overflow)),
                    (with_current, _) => {
                        if b & 0x80 != 0 {
                            acc = with_current;
                            None
                        } else {
                            acc = 0;
                            Some(Ok(with_current))
                        }
                    }
                },
            })
            .collect()
    }
}
