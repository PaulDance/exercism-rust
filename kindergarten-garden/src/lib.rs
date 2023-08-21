/// Returns the desired vector of plants for the given `student` and `diagram`.
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let stdt_idx = 2 * (student.as_bytes()[0] - 'A' as u8) as usize;
    diagram
        .lines()
        .flat_map(|line| {
            let bytes = line.as_bytes();
            [
                cup_to_plant(bytes[stdt_idx] as char),
                cup_to_plant(bytes[stdt_idx + 1] as char),
            ]
        })
        .collect()
}

/// Converts the given `cup` letter to its corresponding plant name.
fn cup_to_plant(cup: char) -> &'static str {
    match cup {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => panic!("Unexpected plant cup letter: '{cup}'."),
    }
}
