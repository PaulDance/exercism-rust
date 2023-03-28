use std::ops;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

/// Executes a binary operation `op` on the `stack`.
///
/// Returns `None` in case the stack does not have enough elements and `Some(())`
/// when everything succeeds.
fn bin_op(stack: &mut Vec<i32>, op: impl FnOnce(i32, i32) -> i32) -> Option<()> {
    // Inverted operand order on the stack.
    let op2 = stack.pop()?;
    let op1 = stack.pop()?;
    stack.push(op(op1, op2));
    Some(())
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::<i32>::new();

    for input in inputs {
        match *input {
            CalculatorInput::Value(val) => stack.push(val),
            CalculatorInput::Add => bin_op(&mut stack, ops::Add::add)?,
            CalculatorInput::Subtract => bin_op(&mut stack, ops::Sub::sub)?,
            CalculatorInput::Multiply => bin_op(&mut stack, ops::Mul::mul)?,
            CalculatorInput::Divide => bin_op(&mut stack, ops::Div::div)?,
        }
    }

    // Stack must contain a single value for the given expression to be correct.
    match &stack[..] {
        &[val] => Some(val),
        _ => None,
    }
}
