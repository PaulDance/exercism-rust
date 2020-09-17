/// In-house parser for english-like math commands.
pub fn answer(command: &str) -> Option<i32> {
    // Remove last '?' and split command into words.
    let words = command
        .trim_end_matches('?')
        .split_whitespace()
        .collect::<Vec<_>>();

    // Reject non math questions early.
    if words[0] != "What" || words[1] != "is" {
        None
    } else {
        let mut i = 2;
        let mut res = None;

        while i < words.len() {
            // Initialize the result with the first correct number.
            if res.is_none() {
                match i32::from_str_radix(words[i], 10) {
                    Err(_) => {
                        return None;
                    }
                    Ok(x) => {
                        res = Some(x);
                    }
                }
            }

            // Compute operation.
            if i + 1 < words.len() {
                let operator: fn(i32, i32) -> i32;
                let next_num: usize;

                // Extract the binary operator from the words.
                match words[i + 1] {
                    "plus" => {
                        operator = |x, y| x + y;
                        next_num = i + 2;
                    }
                    "minus" => {
                        operator = |x, y| x - y;
                        next_num = i + 2;
                    }
                    word => {
                        if i + 2 < words.len() && words[i + 2] == "by" {
                            match word {
                                "multiplied" => {
                                    operator = |x, y| x * y;
                                    next_num = i + 3;
                                }
                                "divided" => {
                                    operator = |x, y| x / y;
                                    next_num = i + 3;
                                }
                                _ => {
                                    return None;
                                }
                            }
                        } else {
                            return None;
                        }
                    }
                }

                // Parse the second operand.
                if next_num < words.len() {
                    match i32::from_str_radix(words[next_num], 10) {
                        Err(_) => {
                            return None;
                        }
                        Ok(x) => {
                            // Apply the operator.
                            res.replace(operator(res.unwrap(), x));
                            i = next_num;
                        }
                    }
                } else {
                    return None;
                }
            } else {
                return res;
            }
        }

        res
    }
}
