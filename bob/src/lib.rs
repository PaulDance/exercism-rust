pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if msg.is_empty() {
        "Fine. Be that way!"
    } else {
        let mut contains_alpha = false;
        let is_upper = msg.chars().filter(|chr| chr.is_alphabetic()).all(|chr| {
            contains_alpha = true;
            chr.is_uppercase()
        }) && contains_alpha;

        match msg.chars().last() {
            Some('?') => {
                if is_upper {
                    "Calm down, I know what I'm doing!"
                } else {
                    "Sure."
                }
            }
            _ => {
                if is_upper {
                    "Whoa, chill out!"
                } else {
                    "Whatever."
                }
            }
        }
    }
}
