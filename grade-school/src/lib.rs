use std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

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
        let student = student.to_string();

        match self.map.get_mut(&grade) {
            None => {
                let mut set = BTreeSet::new();
                set.insert(student);
                self.map.insert(grade, set);
            }
            Some(students) => {
                students.insert(student);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.map.keys().map(|&n| n).collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.map
            .get(&grade)
            .map(|students| students.iter().map(|s| s.clone()).collect())
    }
}
