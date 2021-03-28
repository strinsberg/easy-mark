use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Question {
    pub num: u32,
    pub part: u32,
    pub out_of: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub deduction: f32,
    pub text: String,
    pub names: HashSet<String>,
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
}