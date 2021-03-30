use crate::data::{Comment, Question};
use serde::{Deserialize, Serialize};

/// An assignment for a given course. Collects all the students, questions, and
/// comments together. Stores all question parts with their comments so that
/// they can be easily found and reused for multiple students.
#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// The assignment title.
    pub title: String,
    /// The course the assignment is for.
    pub course: String,
    // All of the students being graded for this assignment.
    students: Vec<String>,
    // Sequential list of question parts, along with their comments. They are
    // sorted by question number and then question part.
    questions: Vec<QuestAndComs>,
    // The next unique id to give to an added comment.
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
    /// The number of students that have been added.
    pub fn num_students(&self) -> u32 {
        self.students.len() as u32
    }

    /// True if a student has been added.
    pub fn student_exists(&self, student: &str) -> bool {
        self.students.iter().any(|s| s == student)
    }

    /// Get the name of the student at the given index.
    /// Panics if the given index is out of range for the number of students.
    pub fn get_student_at(&self, idx: u32) -> String {
        self.students[idx as usize].clone()
    }

    /// Get a vector of all the students that have been added to the assignment.
    pub fn get_students(&self) -> Vec<String> {
        self.students.iter().map(|s| s.clone()).collect()
    }

    /// Add a new student to the assignment.
    /// The students index will be the num_students before adding.
    /// Panics if the student has already been added.
    pub fn add_student(&mut self, student: &str) {
        assert!(!self.student_exists(student));
        self.students.push(student.to_string());
    }

    // Questions /////////////////////////////////////////////////////////////
    /// The number of question parts added to the assignemtn.
    /// I.e. 1.1 1.2 1.3 = 3 questions not 1.
    pub fn num_questions(&self) -> u32 {
        self.questions.len() as u32
    }

    /// True if the given question has been added to the assignment.
    /// Only considers the number and part of the given question. I.e. if
    /// 1.1 has been added out of 5 then if the given question is 1.1 but out
    /// of 10 this will still return true.
    pub fn question_exists(&self, question: &Question) -> bool {
        self.questions
            .iter()
            .any(|qc| qc.question.num == question.num && qc.question.part == question.part)
    }

    /// Gets the question data at the given index.
    /// This is essentially the same as get_student_at where idx is an
    /// implicit id related to the order of the questions. It enables moving
    /// through the question parts without knowing how many parts each
    /// top level question might have.
    pub fn get_question_at(&self, idx: u32) -> Question {
        self.questions[idx as usize].question.clone()
    }

    /// Gets an ordered list of question data objects in a vector.
    pub fn get_questions(&self) -> Vec<Question> {
        self.questions
            .iter()
            .map(|qc| qc.question.clone())
            .collect()
    }

    /// Adds a new question part to the assignment.
    /// Always adds it after existing questions, rather than searching
    /// and inserting it if the number and part would put it out of order.
    /// The added questions index will always be the resutl of num_questions
    /// before the question was added.
    /// Panics if the question has already been added.
    pub fn add_question(&mut self, num: u32, part: u32, out_of: u32) {
        let q = Question { num, part, out_of };
        assert!(!self.question_exists(&q));

        self.questions.push(QuestAndComs {
            question: q,
            comments: Vec::new(),
        });
    }

    // Comments //////////////////////////////////////////////////////////////
    /// Adds a new deduction and comment to the given question for a student.
    pub fn add_comment(
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

    /// Add a student to a comment for the given question.
    /// This represents
    /// adding the comment to a students grade sheet, but the assignment
    /// and comment data represent it the other way.
    pub fn add_to_comment(&mut self, student: &str, question: &Question, id: u64) {
        self.get_comments_mut(question)
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .add_student(student.to_string());
    }

    /// Remove a student from the comment for the given question.
    /// If a comment has no students left it is deleted. This is to keep
    /// the assignment uncluttered with useless or mistaken comments. Though
    /// there may not always be intuitive.
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

    /// Adjusts a comment to have the given deduction and text for the
    /// a question.
    pub fn edit_comment(&mut self, question: &Question, id: u64, deduction: f32, text: String) {
        let mut com = self
            .get_comments_mut(question)
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap();
        com.deduction = deduction;
        com.text = text;
    }

    /// Get a vector of all a student's comments for the given question.
    pub fn students_comments_for(&self, student: &str, question: &Question) -> Vec<Comment> {
        self.get_comments(question)
            .iter()
            .filter(|c| c.has_student(student))
            .map(|c| c.clone())
            .collect()
    }

    /// Get a vector of all comments for the given question that have not
    /// been associated with a student.
    pub fn unused_comments_for(&self, student: &str, question: &Question) -> Vec<Comment> {
        self.get_comments(question)
            .iter()
            .filter(|c| !c.has_student(student))
            .map(|c| c.clone())
            .collect()
    }

    // Helper to get a mutable reference to the vector of comments.
    fn get_comments_mut(&mut self, question: &Question) -> &mut Vec<Comment> {
        &mut self
            .questions
            .iter_mut()
            .find(|qc| &qc.question == question)
            .unwrap()
            .comments
    }

    // Helper to get an immutable reference to the vector of comments.
    fn get_comments(&self, question: &Question) -> &Vec<Comment> {
        &self
            .questions
            .iter()
            .find(|qc| &qc.question == question)
            .unwrap()
            .comments
    }

    // Marks /////////////////////////////////////////////////////////////////
    /// Returns the total number of marks the assignment is out of.
    pub fn out_of(&self) -> u32 {
        self.questions
            .iter()
            .fold(0, |acc, qc| acc + qc.question.out_of)
    }

    /// Returns the total number of marks a student has obtained
    /// on the assignment.
    pub fn students_total(&self, student: &str) -> f32 {
        self.questions.iter().fold(0.0, |acc, qc| {
            acc + self.students_mark_for(student, &qc.question)
        })
    }

    /// Returns the number of marks a student obtained on the given question.
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

// Private struct to hold a pair of a question and a vector of all the
// comments that are associated with it.
#[derive(Debug, Serialize, Deserialize)]
struct QuestAndComs {
    pub question: Question,
    pub comments: Vec<Comment>,
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_test_asn() -> Assignment {
        Assignment::new("Assignment 5".to_string(), "CS 1000".to_string())
    }

    #[test]
    fn it_adds_new_students_and_can_tell_they_exist() {
        let mut asn = make_test_asn();
        asn.add_student("Issac Newton");
        asn.add_student("Albert Einstein");
        asn.add_student("Marie Currie");
        assert_eq!(asn.num_students(), 3);
        assert!(asn.student_exists("Issac Newton"));
        assert!(asn.student_exists("Albert Einstein"));
        assert!(asn.student_exists("Marie Currie"));
    }

    #[test]
    fn it_gets_the_students_in_expected_order() {
        let mut asn = make_test_asn();
        asn.add_student("Issac Newton");
        asn.add_student("Albert Einstein");
        asn.add_student("Marie Currie");
        assert_eq!(asn.get_student_at(0), "Issac Newton".to_string());
        assert_eq!(asn.get_student_at(1), "Albert Einstein".to_string());
        assert_eq!(asn.get_student_at(2), "Marie Currie".to_string());

        let students = asn.get_students();
        assert_eq!(students[0], "Issac Newton".to_string());
        assert_eq!(students[1], "Albert Einstein".to_string());
        assert_eq!(students[2], "Marie Currie".to_string());
    }

    #[test]
    #[should_panic]
    fn it_panics_if_an_existing_student_is_added() {
        let mut asn = make_test_asn();
        asn.add_student("Issac Newton");
        asn.add_student("Issac Newton");
    }

    #[test]
    fn it_adds_new_questions_and_can_tell_they_exist() {
        let mut asn = make_test_asn();
        asn.add_question(1, 1, 5);
        asn.add_question(1, 2, 5);
        asn.add_question(2, 1, 10);
        assert_eq!(asn.num_questions(), 3);

        assert!(asn.question_exists(&Question::new(1, 1, 5)));
        assert!(asn.question_exists(&Question::new(1, 2, 5)));
        assert!(asn.question_exists(&Question::new(2, 1, 10)));

        // Test that it works when num part are the same, but out of is not.
        assert!(asn.question_exists(&Question::new(2, 1, 25)));
    }

    #[test]
    fn it_gets_questions_in_the_correct_order() {
        let mut asn = make_test_asn();
        asn.add_question(1, 1, 5);
        asn.add_question(1, 2, 5);
        asn.add_question(2, 1, 10);
        assert_eq!(asn.get_question_at(0), Question::new(1, 1, 5));
        assert_eq!(asn.get_question_at(1), Question::new(1, 2, 5));
        assert_eq!(asn.get_question_at(2), Question::new(2, 1, 10));

        let questions = asn.get_questions();
        assert_eq!(questions[0], Question::new(1, 1, 5));
        assert_eq!(questions[1], Question::new(1, 2, 5));
        assert_eq!(questions[2], Question::new(2, 1, 10));
    }

    #[test]
    #[should_panic]
    fn it_panics_if_an_existing_question_is_added() {
        let mut asn = make_test_asn();
        asn.add_question(1, 1, 5);
        asn.add_question(1, 1, 10);
    }

    fn asn_with_students_and_questions() -> Assignment {
        let mut asn = make_test_asn();
        asn.add_question(1, 1, 5);
        asn.add_question(1, 2, 5);
        asn.add_question(2, 1, 10);
        asn.add_student("Issac Newton");
        asn.add_student("Albert Einstein");
        asn.add_student("Marie Currie");
        asn
    }

    #[test]
    fn it_adds_new_comment() {
        let q = Question::new(1, 1, 5);
        let mut asn = asn_with_students_and_questions();
        asn.add_comment("Issac Newton", &q, 3.0, "Amateurish work".to_string());

        let comments = asn.students_comments_for("Issac Newton", &q);
        assert_eq!(comments[0].id, 0);
        assert_eq!(comments[0].deduction, 3.0);
        assert_eq!(comments[0].text, "Amateurish work".to_string());
    }

    #[test]
    fn it_adds_student_to_a_comment() {
        let q = Question::new(1, 1, 5);
        let mut asn = asn_with_students_and_questions();
        asn.add_comment("Issac Newton", &q, 3.0, "Amateurish work".to_string());
        asn.add_to_comment("Marie Currie", &q, 0);

        let comments = asn.students_comments_for("Marie Currie", &q);
        assert_eq!(comments[0].id, 0);
        assert_eq!(comments[0].deduction, 3.0);
        assert_eq!(comments[0].text, "Amateurish work".to_string());
    }

    #[test]
    fn it_removes_student_from_a_comment() {
        let q = Question::new(1, 1, 5);
        let mut asn = asn_with_students_and_questions();
        asn.add_comment("Issac Newton", &q, 3.0, "Amateurish work".to_string());
        asn.add_to_comment("Marie Currie", &q, 0);
        asn.remove_from_comment("Issac Newton", &q, 0);

        let comments = asn.students_comments_for("Issac Newton", &q);
        assert_eq!(comments.len(), 0);
        let coms_mc = asn.students_comments_for("Marie Currie", &q);
        assert_eq!(coms_mc.len(), 1);

        // now the comment should be removed completely
        asn.remove_from_comment("Marie Currie", &q, 0);
        let coms_mc = asn.students_comments_for("Marie Currie", &q);
        assert_eq!(coms_mc.len(), 0);

        // use private function to check comments size for the question
        assert_eq!(asn.get_comments(&q).len(), 0);
    }

    #[test]
    fn it_edits_a_comment_globally() {
        let q = Question::new(1, 1, 5);
        let mut asn = asn_with_students_and_questions();
        asn.add_comment("Issac Newton", &q, 3.0, "Amateurish work".to_string());
        asn.add_to_comment("Marie Currie", &q, 0);
        asn.edit_comment(&q, 0, 2.0, "Needs more detail".to_string());

        let coms_in = asn.students_comments_for("Issac Newton", &q);
        assert_eq!(coms_in[0].deduction, 2.0);
        assert_eq!(coms_in[0].text, "Needs more detail".to_string());

        let coms_mc = asn.students_comments_for("Marie Currie", &q);
        assert_eq!(coms_mc[0].deduction, 2.0);
        assert_eq!(coms_mc[0].text, "Needs more detail".to_string());
    }

    #[test]
    fn it_gets_all_used_and_unused_comments_for_a_student_on_a_question() {
        let q = Question::new(1, 1, 5);
        let mut asn = asn_with_students_and_questions();
        asn.add_comment("Issac Newton", &q, 3.0, "Amateurish work".to_string());
        asn.add_comment("Albert Einstein", &q, 5.0, "Not at all correct".to_string());
        asn.add_comment("Marie Currie", &q, 1.0, "On the right track".to_string());

        let coms_has = asn.students_comments_for("Issac Newton", &q);
        assert_eq!(coms_has.len(), 1);
        let coms_unused = asn.unused_comments_for("Issac Newton", &q);
        assert_eq!(coms_unused.len(), 2);
    }
}
