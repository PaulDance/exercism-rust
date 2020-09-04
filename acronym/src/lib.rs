pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|chr: char| !chr.is_ascii_alphabetic() && chr != '\'')
        .filter_map(|word| {
            if word.is_empty() {
                None
            } else {
                let mut first = true;
                let mut lower = false;
                Some(
                    word.chars()
                        .filter_map(|chr| {
                            if first {
                                first = false;
                                Some(chr.to_ascii_uppercase())
                            } else if chr.is_ascii_uppercase() {
                                if lower {
                                    lower = false;
                                    Some(chr.to_ascii_uppercase())
                                } else {
                                    None
                                }
                            } else {
                                lower = true;
                                None
                            }
                        })
                        .collect::<String>(),
                )
            }
        })
        .collect()
}
