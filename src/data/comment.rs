use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A comment for grading assignments. Rather than being applied to each
/// student, any student that has the comment is added to the comment so that
/// it can be used more than once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// An id that should be made unique by the owner
    pub id: u64,
    /// The amount to deduct from the total of the question
    pub deduction: f32,
    /// The comment text
    pub text: String,
    // The names of the students that this comment should be applied to
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

    /// Returns true if the given student has been added to the comment.
    pub fn has_student(&self, student: &str) -> bool {
        self.names.contains(student)
    }

    /// Adds a new student to the comment.
    /// Panics if the student has already been added.
    //  Wether or not it is a problem to add or remove existing students should
    //  probably be the responsibility of the caller, even though I want this
    //  behaviour in the app, as if it happens something has gone wrong somewhere.
    pub fn add_student(&mut self, student: String) {
        assert!(!self.has_student(&student));
        self.names.insert(student);
    }

    /// Removes the given student from the comment.
    pub fn remove_student(&mut self, student: &str) {
        self.names.remove(student);
    }

    /// Returns true if there are no students added to the comment.
    pub fn empty(&self) -> bool {
        self.names.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_knows_which_students_it_has() {
        let comment = Comment::new(
            0,
            3.0,
            "Not an answer.".to_string(),
            "Albert Einstein".to_string(),
        );

        assert!(comment.has_student("Albert Einstein"));
        assert!(!comment.has_student("Issac Newton"));
    }

    #[test]
    fn it_adds_a_student() {
        let mut comment = Comment::new(
            0,
            3.0,
            "Not an answer.".to_string(),
            "Albert Einstein".to_string(),
        );

        comment.add_student("Issac Newton".to_string());
        assert!(comment.has_student("Issac Newton"));
    }

    #[test]
    #[should_panic]
    fn it_panics_if_a_student_has_already_been_added() {
        let mut comment = Comment::new(
            0,
            3.0,
            "Not an answer.".to_string(),
            "Albert Einstein".to_string(),
        );

        comment.add_student("Albert Einstein".to_string());
    }

    #[test]
    fn it_removes_a_student() {
        let mut comment = Comment::new(
            0,
            3.0,
            "Not an answer.".to_string(),
            "Albert Einstein".to_string(),
        );

        comment.remove_student("Albert Einstein");
        assert!(!comment.has_student("Albert Einstein"));
    }

    #[test]
    fn it_knows_when_it_is_empty() {
        let mut comment = Comment::new(
            0,
            3.0,
            "Not an answer.".to_string(),
            "Albert Einstein".to_string(),
        );

        assert!(!comment.empty());
        comment.remove_student("Albert Einstein");
        assert!(comment.empty());
    }
}
