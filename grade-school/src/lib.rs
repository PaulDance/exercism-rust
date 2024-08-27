use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    map: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // Reject already-recorded students.
        if !self.map.values().any(|students| students.contains(student)) {
            self.map
                .entry(grade)
                .or_default()
                .insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.map.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<&str> {
        self.map
            .get(&grade)
            .map(|students| students.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}
