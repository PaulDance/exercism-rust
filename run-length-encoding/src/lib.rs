/// Returns the run-length encoding of the given string.
pub fn encode(source: &str) -> String {
    match source.chars().next() {
        None => String::new(),
        Some(mut running) => {
            let mut count = 1;
            let mut rle = String::new();

            for chr in source.chars().skip(1) {
                if chr == running {
                    count += 1;
                } else {
                    if count > 1 {
                        rle.push_str(count.to_string().as_str());
                    }

                    rle.push(running);
                    running = chr;
                    count = 1;
                }
            }

            if count > 1 {
                rle.push_str(count.to_string().as_str());
            }

            rle.push(running);
            rle
        }
    }
}

/// Returns the run-length decoding of the given string.
pub fn decode(source: &str) -> String {
    let mut rld = String::new();
    let mut num = String::new();

    for chr in source.chars() {
        if chr.is_digit(10) {
            num.push(chr);
        } else if num.is_empty() {
            rld.push(chr);
        } else {
            rld.push_str(
                chr.to_string()
                    .as_str()
                    .repeat(num.parse::<usize>().unwrap() - 1)
                    .as_str(),
            );
            rld.push(chr);
            num.clear();
        }
    }

    rld
}
