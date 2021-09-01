use std::collections::{btree_set::BTreeSet, btree_map::BTreeMap};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>
}

impl School {
    pub fn new() -> School {
        School{grades: BTreeMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.grades.contains_key(&grade) {
            self.grades.insert(grade, BTreeSet::new());
        }
        if let Some(grade) = self.grades.get_mut(&grade) {
            grade.insert(student.to_owned());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(students) = self.grades.get(&grade) {
            Some(students.iter().cloned().collect())
        } else {
            None
        }
    }
}
