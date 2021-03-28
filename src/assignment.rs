use crate::comment::Comment;
use crate::comment::Question;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub title: String,
    pub course: String,
    pub students: Vec<String>,
    pub questions: Vec<Question>,
    pub comments: HashMap<Question, Vec<Comment>>,
    next_id: u64,
}

impl Assignment {
    pub fn new(title: String, course: String) -> Assignment {
        Assignment {
            title,
            course,
            students: Vec::new(),
            questions: Vec::new(),
            comments: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn new_question(&mut self, num: u32, part: u32, out_of: u32) {
        let q = Question { num, part, out_of };
        assert!(!self.comments.contains_key(&q));

        self.questions.push(q.clone());
        self.comments.insert(q, Vec::new());
    }

    pub fn new_comment(
        &mut self,
        student: &str,
        question: &Question,
        deduction: f32,
        text: String,
    ) {
        let com = Comment::new(self.next_id, deduction, text, student.to_string());
        self.next_id += 1;

        self.comments.get_mut(question).unwrap().push(com);
    }

    pub fn add_comment_to(&mut self, student: &str, question: &Question, id: u64) {
        self.comments
            .get_mut(question)
            .unwrap()
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .names
            .insert(student.to_string());
    }

    pub fn remove_comment_from(&mut self, student: &str, question: &Question, id: u64) {
        self.comments
            .get_mut(question)
            .unwrap()
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .names
            .remove(student);
        // if the comment has 0 names now, remove... names.retain(|c| c != com); ???
    }

    pub fn edit_comment(&mut self, question: &Question, id: u64, deduction: f32, text: String) {
        let mut com = self
            .comments
            .get_mut(question)
            .unwrap()
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap();
        com.deduction = deduction;
        com.text = text;
    }

    pub fn out_of(&self) -> u32 {
        self.questions.iter().fold(0, |acc, q| acc + q.out_of)
    }

    pub fn question_comments(&self, student: &str, question: &Question) -> Vec<Comment> {
        self.comments
            .get(question)
            .unwrap()
            .iter()
            .filter(|c| c.names.contains(student))
            .map(|c| c.clone())
            .collect()
    }

    pub fn question_mark(&self, student: &str, question: &Question) -> f32 {
        let total = question.out_of as f32;
        let deducted = self
            .question_comments(student, question)
            .iter()
            .fold(0.0, |acc, c| acc + c.deduction);

        let res = total - deducted;
        if res > 0.0 {
            res
        } else {
            0.0
        }
    }

    pub fn question_comments_unused(&self, student: &str, question: &Question) -> Vec<Comment> {
        self.comments
            .get(question)
            .unwrap()
            .iter()
            .filter(|c| !c.names.contains(student))
            .map(|c| c.clone())
            .collect()
    }

    pub fn total_mark(&self, student: &str) -> f32 {
        self.questions
            .iter()
            .fold(0.0, |acc, q| acc + self.question_mark(student, q))
    }
}
