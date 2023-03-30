/// Returns the desired vector of actions given the input number.
pub fn actions(n: u8) -> Vec<&'static str> {
    actions_order(n)
        .filter(|&i| n & (1 << i) != 0)
        .map(pos_to_action)
        .collect()
}

/// Returns the appropriate bit position iterator given the input number.
fn actions_order(n: u8) -> Box<dyn Iterator<Item = u8>> {
    // Boxing is required in order to federate the different iterator types.
    if n & (1 << 4) == 0 {
        Box::new(0..=3)
    } else {
        Box::new((0..=3).rev())
    }
}

/// Converts the given bit position `i` to an action string.
///
/// Panics in case a position greater than 3 is received.
fn pos_to_action(i: u8) -> &'static str {
    match i {
        0 => "wink",
        1 => "double blink",
        2 => "close your eyes",
        3 => "jump",
        _ => panic!("Unexpected bit position: {i}."),
    }
}
