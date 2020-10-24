use std::collections::HashMap;
use std::convert::TryFrom;

use maplit::hashmap;

/// The only possible value type considered here.
pub type Value = i32;
/// Custom result.
pub type ForthResult = Result<(), Error>;

/// Custom error enumeration.
#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

/// The desired type.
pub struct Forth {
    /// The value stack.
    stack: Vec<Value>,
    /// Dictionary mapping word names to lists of operations.
    words: HashMap<String, Procedure>,
}

impl Forth {
    /// Builds a new Forth interpreter with its word dictionary already populated
    /// with the supported built-in words so they may be overidden later on.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            words: hashmap! {
                "+".to_string() => Procedure::single(Operation::Add),
                "-".to_string() => Procedure::single(Operation::Substract),
                "*".to_string() => Procedure::single(Operation::Multiply),
                "/".to_string() => Procedure::single(Operation::Divide),
                "dup".to_string() => Procedure::single(Operation::Duplicate),
                "drop".to_string() => Procedure::single(Operation::Drop),
                "swap".to_string() => Procedure::single(Operation::Swap),
                "over".to_string() => Procedure::single(Operation::Over),
            },
        }
    }

    /// Returns a clone of the interpreter's memory stack.
    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    /// Parses and evaluates a given `input` string of Forth code.
    pub fn eval(&mut self, input: &str) -> ForthResult {
        self.exec(Procedure::try_from(input)?)
    }

    /// Executes an already parsed procedure.
    fn exec(&mut self, proc: Procedure) -> ForthResult {
        // Evaluate operations linearly thanks to the postfix notation.
        for op in proc.ops {
            match op {
                // Arithmetics.
                Operation::Add => {
                    let last = self.pop_stack()?;
                    let next = self.pop_stack()?;
                    self.stack.push(next + last);
                }
                Operation::Substract => {
                    let last = self.pop_stack()?;
                    let next = self.pop_stack()?;
                    self.stack.push(next - last);
                }
                Operation::Multiply => {
                    let last = self.pop_stack()?;
                    let next = self.pop_stack()?;
                    self.stack.push(next * last);
                }
                Operation::Divide => {
                    let last = self.pop_stack().and_then(|x| {
                        if x == 0 {
                            Err(Error::DivisionByZero)
                        } else {
                            Ok(x)
                        }
                    })?;
                    let next = self.pop_stack()?;
                    self.stack.push(next / last);
                }
                // Memory.
                Operation::Duplicate => {
                    self.stack
                        .push(*self.stack.last().ok_or(Error::StackUnderflow)?);
                }
                Operation::Drop => {
                    self.pop_stack()?;
                }
                Operation::Swap => {
                    let last = self.pop_stack()?;
                    let next = self.pop_stack()?;
                    self.stack.push(last);
                    self.stack.push(next);
                }
                Operation::Over => {
                    let last = self.pop_stack()?;
                    let next = self.pop_stack()?;
                    self.stack.push(next.clone());
                    self.stack.push(last);
                    self.stack.push(next);
                }
                Operation::Push(value) => {
                    self.stack.push(value);
                }
                // Words.
                Operation::Define(name, mut word) => {
                    word.inline_calls(&self.words);
                    self.words.insert(name, word);
                }
                Operation::Call(word_name) => match self.words.get(&word_name).cloned() {
                    None => return Err(Error::UnknownWord),
                    Some(word) => {
                        self.exec(word)?;
                    }
                },
            }
        }

        Ok(())
    }

    /// Convenience function: pops the stack in an `Ok` it not empty, else `Err`.
    fn pop_stack(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }
}

/// Represents a Forth operation.
#[derive(Debug, Clone)]
enum Operation {
    // Arithmetics
    Add,
    Substract,
    Multiply,
    Divide,
    // Memory
    Duplicate,
    Drop,
    Swap,
    Over,
    /// When a literal number is parsed.
    Push(Value),
    /// An already parsed but no yet evaluated word definition.
    Define(String, Procedure),
    /// A word call.
    Call(String),
}

/// Provides a constructor.
impl From<&str> for Operation {
    /// Parses an operation from the given string.
    fn from(value: &str) -> Self {
        // Turn everything lowercase.
        let value = value.to_ascii_lowercase();

        // Push a number if it is one, otherwise make a call.
        if value.chars().all(|chr| chr.is_ascii_digit()) {
            Self::Push(Value::from_str_radix(&value, 10).unwrap())
        } else {
            Self::Call(value)
        }
    }
}

/// Represents a procedure as a series of operations.
#[derive(Debug, Clone)]
struct Procedure {
    /// The series of operations.
    ops: Vec<Operation>,
}

/// Provides a constructor.
impl TryFrom<&str> for Procedure {
    type Error = Error;

    /// Attempts to convert the given string to a procedure by splitting it by
    /// whitespace into elements and feed it to `try_from_elements`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_elements(&mut value.split_ascii_whitespace(), false)
    }
}

impl Procedure {
    /// Attempts to build a procedure from an iterator of strings. `in_def` is
    /// to indicate whether the call is in a word definition context.
    fn try_from_elements<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
        in_def: bool,
    ) -> Result<Self, Error> {
        let mut ops = Vec::with_capacity(iter.size_hint().0);
        let mut elt_met = false;
        let mut sc_met = false;

        while let Some(elt) = iter.next() {
            // Stop if semi-colon is encountered.
            if in_def && elt == ";" {
                sc_met = true;
                break;
            } else if elt == ":" {
                // Word definition mode.
                match iter.next() {
                    // Reject empty name.
                    None => return Err(Error::InvalidWord),
                    Some(name) => {
                        // Reject invalid names.
                        if name.starts_with(|chr: char| chr.is_ascii_digit()) {
                            return Err(Error::InvalidWord);
                        } else {
                            // Parse word body into a new define operation.
                            ops.push(Operation::Define(
                                name.to_ascii_lowercase(),
                                Self::try_from_elements(iter, true)?,
                            ));
                        }
                    }
                }
            } else {
                // Otherwise, just parse the element as an operation.
                elt_met = true;
                ops.push(Operation::from(elt));
            }
        }

        // Reject empty word definition bodies and absences of semi-colon.
        if in_def && !(elt_met && sc_met) {
            Err(Error::InvalidWord)
        } else {
            Ok(Self { ops })
        }
    }

    /// Convenience function: builds a procedure from a single operation.
    fn single(op: Operation) -> Self {
        Self { ops: vec![op] }
    }

    /// Inlines word calls in the procedure when already defined in the given
    /// word dictionary.
    fn inline_calls(&mut self, words: &HashMap<String, Self>) {
        let mut new_ops = Vec::new();

        // If a call operation is encountered and the called word already
        // exists, inline its content in the current procedure.
        for op in self.ops.iter() {
            if let Operation::Call(name) = op {
                if let Some(mut word) = words.get(name).cloned() {
                    // Here could be a recursive call on `word`, but it
                    // is not necessary, as inlining occurs ASAP.
                    new_ops.append(&mut word.ops);
                    continue;
                }
            }

            // Let the operation pass as-is otherwise.
            new_ops.push(op.clone());
        }

        self.ops = new_ops;
    }
}
