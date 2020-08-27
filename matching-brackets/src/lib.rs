fn is_bracket(chr: &char) -> bool {
    is_opening_bracket(*chr) || is_closing_bracket(*chr)
}

fn is_opening_bracket(chr: char) -> bool {
    match chr {
        '(' | '[' | '{' => true,
        _ => false,
    }
}

fn is_closing_bracket(chr: char) -> bool {
    match chr {
        ')' | ']' | '}' => true,
        _ => false,
    }
}

fn matching_bracket(chr: char) -> char {
    match chr {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("Not a bracket: '{}'.", chr),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();

    for new in string.chars().filter(is_bracket) {
        match stack.last() {
            Some(&last) => {
                if is_closing_bracket(new) {
                    if new != matching_bracket(last) {
                        return false;
                    } else {
                        stack.pop();
                    }
                } else {
                    stack.push(new);
                }
            }
            None => {
                if is_closing_bracket(new) {
                    return false;
                } else {
                    stack.push(new);
                }
            }
        }
    }

    stack.is_empty()
}
