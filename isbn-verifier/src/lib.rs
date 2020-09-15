/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() == 0 {
        false
    } else {
        let last = isbn.chars().last().unwrap();

        if !last.is_digit(10) && last != 'X' {
            false
        } else {
            let no_dashes = isbn[..isbn.len() - 1].replace(|chr: char| !chr.is_digit(10), "")
                + last.to_string().as_str();

            if no_dashes.len() != 10 {
                false
            } else {
                no_dashes.char_indices().fold(0, |acc, (i, chr)| {
                    acc + (no_dashes.len() - i)
                        * match chr {
                            'X' => 10,
                            _ => chr.to_digit(10).unwrap() as usize,
                        }
                }) % (no_dashes.len() + 1)
                    == 0
            }
        }
    }
}
