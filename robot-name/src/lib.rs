//! Thread-safe implementation with a static storage.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use rand::prelude::*;

/// The length of the names to generate.
const NAMES_LENGTH: usize = 5;
/// The number of letters to put at the beginning of names.
const NAMES_LETTERS: usize = 2;
/// The number of digits to put at the end of names.
const NAMES_DIGITS: usize = 3;

// lazy_static is necessary here in order to use non-const functions.
lazy_static! {
    /// Central storage for the previously generated names.
    ///
    ///  * Arc: thread-safe mutable access
    ///  * Mutex: mutual exclusion of accesses
    ///  * HashSet: fast search, insertion and removal
    ///  * String: owned string type
    static ref PREV_NAMES: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
}

/// A named robot.
pub struct Robot {
    name: String,
}

impl Robot {
    /// Builds a new `Robot` by generating a new name.
    pub fn new() -> Self {
        Self {
            name: Self::gen_name(),
        }
    }

    /// Returns a reference to the robot's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Wipes and regenerates the robot's name.
    pub fn reset_name(&mut self) {
        let old = self.name.clone();
        self.name = Self::gen_name();
        PREV_NAMES.lock().unwrap().remove(&old);
    }

    /// Generates a new robot name that is not currently used.
    fn gen_name() -> String {
        let mut name = String::with_capacity(NAMES_LENGTH);
        let mut rng = thread_rng();

        // do
        loop {
            name.clear();

            // Generate the letters,
            for _ in 0..NAMES_LETTERS {
                name.push(('A'..='Z').choose(&mut rng).unwrap());
            }

            // and digits,
            for _ in 0..NAMES_DIGITS {
                name.push(('0'..='9').choose(&mut rng).unwrap());
            }

            let mut prev_names = PREV_NAMES.lock().unwrap();

            // while it already exists.
            if !prev_names.contains(&name) {
                prev_names.insert(name.clone());
                break;
            }
        }

        name
    }
}
