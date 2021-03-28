use crate::data::{Comment, Question};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub title: String,
    pub course: String,
    students: Vec<String>,
    questions: Vec<QuestAndComs>,
    next_id: u64,
}

impl Assignment {
    pub fn new(title: String, course: String) -> Assignment {
        Assignment {
            title,
            course,
            students: Vec::new(),
            questions: Vec::new(),
            next_id: 0,
        }
    }

    // Students //////////////////////////////////////////////////////////////
    pub fn num_students(&self) -> u32 {
        self.students.len() as u32
    }

    pub fn student_exists(&self, student: &str) -> bool {
        self.students.iter().any(|s| s == student)
    }

    pub fn get_student_at(&self, idx: u32) -> String {
        self.students[idx as usize].clone()
    }

    pub fn get_students(&self) -> Vec<String> {
        self.students.iter().map(|s| s.clone()).collect()
    }

    pub fn add_student(&mut self, student: &str) {
        assert!(!self.student_exists(student));
        self.students.push(student.to_string());
    }

    // Questions /////////////////////////////////////////////////////////////
    pub fn num_questions(&self) -> u32 {
        self.questions.len() as u32
    }

    pub fn question_exists(&self, question: &Question) -> bool {
        self.questions.iter().any(|qc| &qc.question == question)
    }

    pub fn get_question_at(&self, idx: u32) -> Question {
        self.questions[idx as usize].question.clone()
    }

    pub fn get_questions(&self) -> Vec<Question> {
        self.questions
            .iter()
            .map(|qc| qc.question.clone())
            .collect()
    }

    pub fn new_question(&mut self, num: u32, part: u32, out_of: u32) {
        let q = Question { num, part, out_of };
        assert!(!self.question_exists(&q));

        self.questions.push(QuestAndComs {
            question: q,
            comments: Vec::new(),
        });
    }

    // Comments //////////////////////////////////////////////////////////////
    pub fn new_comment(
        &mut self,
        student: &str,
        question: &Question,
        deduction: f32,
        text: String,
    ) {
        let com = Comment::new(self.next_id, deduction, text, student.to_string());
        self.next_id += 1;

        self.get_comments_mut(question).push(com);
    }

    pub fn add_to_comment(&mut self, student: &str, question: &Question, id: u64) {
        self.get_comments_mut(question)
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .add_student(student.to_string());
    }

    pub fn remove_from_comment(&mut self, student: &str, question: &Question, id: u64) {
        let (id, empty) = {
            let com = self
                .get_comments_mut(question)
                .iter_mut()
                .find(|c| c.id == id)
                .unwrap();

            com.remove_student(student);
            (com.id, com.empty())
        };
        if empty {
            self.get_comments_mut(question).retain(|c| c.id != id);
        }
    }

    pub fn edit_comment(&mut self, question: &Question, id: u64, deduction: f32, text: String) {
        let mut com = self
            .get_comments_mut(question)
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap();
        com.deduction = deduction;
        com.text = text;
    }

    pub fn students_comments_for(&self, student: &str, question: &Question) -> Vec<Comment> {
        self.get_comments(question)
            .iter()
            .filter(|c| c.has_student(student))
            .map(|c| c.clone())
            .collect()
    }

    pub fn unused_comments_for(&self, student: &str, question: &Question) -> Vec<Comment> {
        self.get_comments(question)
            .iter()
            .filter(|c| !c.has_student(student))
            .map(|c| c.clone())
            .collect()
    }

    fn get_comments_mut(&mut self, question: &Question) -> &mut Vec<Comment> {
        &mut self
            .questions
            .iter_mut()
            .find(|qc| &qc.question == question)
            .unwrap()
            .comments
    }

    fn get_comments(&self, question: &Question) -> &Vec<Comment> {
        &self
            .questions
            .iter()
            .find(|qc| &qc.question == question)
            .unwrap()
            .comments
    }

    // Marks /////////////////////////////////////////////////////////////////
    pub fn out_of(&self) -> u32 {
        self.questions
            .iter()
            .fold(0, |acc, qc| acc + qc.question.out_of)
    }

    pub fn students_total(&self, student: &str) -> f32 {
        self.questions.iter().fold(0.0, |acc, qc| {
            acc + self.students_mark_for(student, &qc.question)
        })
    }

    pub fn students_mark_for(&self, student: &str, question: &Question) -> f32 {
        let total = question.out_of as f32;
        let deducted = self
            .students_comments_for(student, question)
            .iter()
            .fold(0.0, |acc, c| acc + c.deduction);

        let res = total - deducted;
        if res > 0.0 {
            res
        } else {
            0.0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct QuestAndComs {
    pub question: Question,
    pub comments: Vec<Comment>,
}
