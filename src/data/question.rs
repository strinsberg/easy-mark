use serde::{Deserialize, Serialize};

/// A part of a question on an assignment.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Question {
    /// The number of the question.
    pub num: u32,
    /// The question part. I.e 1.1, 1.2, etc.
    pub part: u32,
    /// The number of marks the question is out of.
    pub out_of: u32,
}

impl Question {
    pub fn new(num: u32, part: u32, out_of: u32) -> Self {
        Self { num, part, out_of }
    }
}
