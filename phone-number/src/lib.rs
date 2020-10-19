pub fn number(user_number: &str) -> Option<String> {
    // Extract digits.
    let partial_clean = user_number
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();
    let bytes = partial_clean.as_bytes();

    // Reject small results or incorrect country code.
    if partial_clean.len() < 10 || partial_clean.len() == 11 && bytes[0] as char != '1' {
        None
    } else {
        // Process 11 or 10-digits long numbers the same way.
        let shift = partial_clean.len() - 10;

        // Reject incorrect area or exchange codes.
        if [0, 3]
            .iter()
            .any(|index| ['0', '1'].contains(&(bytes[shift + index] as char)))
        {
            None
        } else {
            Some(partial_clean[shift..].to_string())
        }
    }
}
