use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub deduction: f32,
    pub text: String,
    names: HashSet<String>,
}

impl Comment {
    pub fn new(id: u64, deduction: f32, text: String, student: String) -> Comment {
        let mut names = HashSet::new();
        names.insert(student);
        Comment {
            id,
            deduction,
            text,
            names,
        }
    }

    pub fn has_student(&self, student: &str) -> bool {
        self.names.contains(student)
    }

    pub fn add_student(&mut self, student: String) {
        assert!(!self.has_student(&student));
        self.names.insert(student);
    }

    pub fn remove_student(&mut self, student: &str) {
        self.names.remove(student);
    }
}
