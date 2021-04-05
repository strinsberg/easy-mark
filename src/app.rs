use crate::data::{Assignment, Question};
use crate::io::{FileOps, View};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct App<V: View, F: FileOps> {
    assignment: Assignment,
    student: String,
    student_idx: u32,
    question: Question,
    question_idx: u32,
    view: PhantomData<V>,
    file_ops: PhantomData<F>,
}

impl<V: View, F: FileOps> App<V, F> {
    pub fn new() -> Self {
        Self {
            assignment: Assignment::new("default".to_string(), "none".to_string()),
            student: "none".to_string(),
            student_idx: 0,
            question: Question {
                num: 1,
                part: 1,
                out_of: 0,
            },
            question_idx: 0,
            view: PhantomData,
            file_ops: PhantomData,
        }
    }

    // Run and Main Menu /////////////////////////////////////////////////////
    pub fn run(&mut self) {
        let header = "Main Menu".to_string();
        let menu = vec![
            "New Assignment".to_string(),
            "Load Assignment".to_string(),
            "Quit".to_string(),
        ];

        V::clear_screen();
        loop {
            let choice = V::show_menu(&header, &menu);
            match choice {
                1 => {
                    self.assignment = V::create_assignment();
                    self.new_student();
                }
                2 => match V::load_assignment() {
                    Some(asn) => self.set_assignment(asn),
                    None => continue,
                },
                _ => break,
            }

            self.question = self.assignment.get_question_at(self.question_idx);
            self.asn_menu();
        }
    }

    fn set_assignment(&mut self, assignment: Assignment) {
        self.assignment = assignment;
        self.student_idx = 0;
        self.student = self.assignment.get_student_at(self.student_idx);
        self.question_idx = 0;
        self.question = self.assignment.get_question_at(self.question_idx);
    }

    // Assignment menu ///////////////////////////////////////////////////////
    fn asn_menu(&mut self) {
        let menu = vec![
            "Start Grading".to_string(),
            "New Student".to_string(),
            "Next Student".to_string(),
            "Prev Student".to_string(),
            "Display Current Grade Sheet".to_string(),
            "Dump Grade Sheet To Latex".to_string(),
            "Dump ALL To Latex".to_string(),
            "Back".to_string(),
        ];

        loop {
            let header = format!("{} Menu ({})", self.assignment.title, self.student);

            let choice = V::show_menu(&header, &menu);
            match choice {
                1 => self.question_menu(),
                2 => self.new_student(),
                3 => self.change_student(1),
                4 => self.change_student(-1),
                5 => V::show_grade_sheet(&self.assignment, &self.student),
                6 => F::save_latex_grade_sheet(&self.assignment, &self.student),
                7 => F::save_all_latex_grade_sheets(&self.assignment),
                _ => break,
            }
        }
    }

    fn new_student(&mut self) {
        self.student = V::new_student(&self.assignment);
        self.assignment.add_student(&self.student);
        self.student_idx = self.assignment.num_students() - 1;
        F::save_assignment(&self.assignment);
    }

    fn change_student(&mut self, dx: i32) {
        if dx >= 0 {
            let x = (self.student_idx + 1) % self.assignment.num_students();
            self.student_idx = x;
        } else if self.student_idx == 0 {
            self.student_idx = self.assignment.num_students() - 1;
        } else {
            self.student_idx -= 1;
        }
        self.student = self.assignment.get_student_at(self.student_idx);
    }

    // Question Menu /////////////////////////////////////////////////////////
    fn question_menu(&mut self) {
        let menu = vec![
            "Add New Comment".to_string(),
            "Add Existing Comment".to_string(),
            "Edit Comment".to_string(),
            "Remove Comment".to_string(),
            "Next Question".to_string(),
            "Prev Question".to_string(),
            "Back".to_string(),
        ];

        loop {
            V::show_question_info(&self.assignment, &self.student, &self.question);
            let header = format!("Grading: {}", self.student);
            let choice = V::show_menu(&header, &menu);
            match choice {
                1 => self.add_new_comment(),
                2 => self.add_existing_comment(),
                3 => self.edit_comment(),
                4 => self.remove_comment(),
                5 => self.change_question(1),
                6 => self.change_question(-1),
                _ => break,
            }
        }
    }

    fn add_new_comment(&mut self) {
        match V::new_comment() {
            Some((deduct, text)) => {
                self.assignment
                    .add_comment(&self.student, &self.question, deduct, text);
                F::save_assignment(&self.assignment);
            }
            _ => (),
        }
    }

    fn add_existing_comment(&mut self) {
        match V::add_existing_comment(&self.assignment, &self.student, &self.question) {
            Some(id) => {
                self.assignment
                    .add_to_comment(&self.student, &self.question, id);
                F::save_assignment(&self.assignment);
            }
            _ => (),
        }
    }

    fn edit_comment(&mut self) {
        match V::edit_comment(&self.assignment, &self.student, &self.question) {
            Some((deduct, text, id)) => {
                self.assignment
                    .edit_comment(&self.question, id, deduct, text);
                F::save_assignment(&self.assignment);
            }
            _ => (),
        };
    }

    fn remove_comment(&mut self) {
        match V::remove_comment(&self.assignment, &self.student, &self.question) {
            Some(id) => {
                self.assignment
                    .remove_from_comment(&self.student, &self.question, id);
                F::save_assignment(&self.assignment);
            }
            _ => (),
        }
    }

    fn change_question(&mut self, dx: i32) {
        if dx >= 0 {
            let x = (self.question_idx + 1) % self.assignment.num_questions();
            self.question_idx = x;
        } else if self.question_idx == 0 {
            self.question_idx = self.assignment.num_questions() - 1;
        } else {
            self.question_idx -= 1;
        }
        self.question = self.assignment.get_question_at(self.question_idx);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::io::MockFileOps;
    use crate::io::MockView;
    use mockall::Sequence;

    fn make_test_asn() -> Assignment {
        let q1 = Question::new(1, 1, 5);
        let q2 = Question::new(1, 2, 5);
        let q3 = Question::new(2, 1, 10);

        let mut asn = Assignment::new("Assignment 5".to_string(), "CS 1000".to_string());
        asn.add_question(q1.num, q1.part, q1.out_of);
        asn.add_question(q2.num, q2.part, q2.out_of);
        asn.add_question(q3.num, q3.part, q3.out_of);

        asn.add_student("Issac Newton");
        asn.add_student("Albert Einstein");
        asn.add_student("Marie Currie");

        asn.add_comment("Albert Einstein", &q1, 3.0, "Amateurish work".to_string());
        asn.add_comment("Albert Einstein", &q2, 5.0, "Not correct".to_string());
        asn.add_comment("Marie Currie", &q1, 1.0, "On the right track".to_string());

        asn.add_comment("Issac Newton", &q2, 1.5, "Amateurish work".to_string());
        asn.add_comment("Issac Newton", &q2, 2.0, "Try harder".to_string());
        asn.add_comment("Issac Newton", &q3, 3.0, "Mind the apples".to_string());

        asn
    }

    #[test]
    fn it_sets_a_new_assignment() {
        let mut app = App::<MockView, MockFileOps>::new();
        let asn = make_test_asn();
        app.set_assignment(asn);

        assert_eq!(app.assignment.title, "Assignment 5".to_string());
        assert_eq!(app.assignment.course, "CS 1000".to_string());
        assert_eq!(app.student_idx, 0);
        assert_eq!(app.question_idx, 0);
        assert_eq!(app.assignment.get_student_at(0), "Issac Newton");
        assert_eq!(app.assignment.get_question_at(0), Question::new(1, 1, 5));
    }

    #[test]
    fn the_assignment_menu_displays_grade_sheet_for_5() {
        // Remove this test and just test the other functions. The menus are
        // too suceptible to changes and doing the loops is a pain in tests.
        // the logic for the tasks is more important for the running of the app.
        // Perhaps this logic can be used to test the io stuff, where we just
        // test for the actions and ignore all the output, and maybe input.
        let mut seq = Sequence::new();
        let ctx_sm = MockView::show_menu_context();
        let ctx_sgs = MockView::show_grade_sheet_context();

        ctx_sm
            .expect()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(5u32);
        ctx_sgs
            .expect()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        ctx_sm
            .expect()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(8u32);

        let mut app = App::<MockView, MockFileOps>::new();
        app.asn_menu();
    }
}
