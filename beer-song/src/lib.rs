/// Puts into words the given number of bottles.
fn count_bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

/// Transforms the given number of bottles into the word between "Take" and "down".
fn take_down(n: u32) -> &'static str {
    match n {
        1 => "it",
        _ => "one",
    }
}

/// Builds and returns the 99 bottles' n-th verse.
pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo \
              to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        _ => format!(
            "{0} of beer on the wall, {0} of beer.\nTake {1} \
             down and pass it around, {2} of beer on the wall.\n",
            count_bottles(n),
            take_down(n),
            count_bottles(n - 1)
        ),
    }
}

/// Builds and returns the 99 bottles' verses from start to end included.
pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().fold(String::new(), |acc, n| {
        acc + if n == start { "" } else { "\n" } + &verse(n)
    })
}
