use crate::assignment::Assignment;
use crate::comment::Comment;
use crate::comment::Question;
use crate::display;
use crate::latex;

pub struct App {
    assignment: Assignment,
    student: String,
    student_idx: u32,
    question: Question,
    question_idx: u32,
}

impl App {
    pub fn new() -> App {
        App {
            assignment: Assignment::new(
                "default".to_string(),
                "none".to_string()
            ),
            student: "none".to_string(),
            student_idx: 0,
            question: Question {
                num: 1,
                part: 1,
                out_of: 0
            },
            question_idx: 0,
        }
    }

    pub fn run(&mut self) {
        let header = "Main Menu".to_string();
        let menu = vec![
            "New Assignment".to_string(),
            "Load Assignment".to_string(),
            "Quit".to_string()
        ];

        display::clear_screen();
        let choice = display::get_menu_choice(&header, &menu);
        match choice {
            1 => {
                self.assignment = display::create_new_assignment();
                self.new_student();
            },
            2 => match display::load_assignment() {
                Some(asn) => self.assignment = asn,
                None => return ()
            },
            _ => return ()
        }

        self.question = self.assignment.questions[self.question_idx as usize].clone();
        self.asn_menu();
    }

    fn asn_menu(&mut self) {
        let menu = vec![
            "Start Grading".to_string(),
            "New Student".to_string(),
            "Next Student".to_string(),
            "Prev Student".to_string(),
            "Display Current Grade Sheet".to_string(),
            "Dump Grade Sheet To Latex".to_string(),
            "Dump ALL To Latex".to_string(),
            "Quit".to_string()
        ];

        loop {
            let header = format!(
                "{} Menu ({})", 
                self.assignment.title,
                self.student
            );

            let choice = display::get_menu_choice(&header, &menu);
            match choice {
                1 => self.question_menu(),
                2 => self.new_student(),
                3 => self.change_student(1),
                4 => self.change_student(-1),
                5 => display::grade_sheet(
                    &self.assignment,
                    &self.student
                ),
                6 => latex::dump_grade_sheet(
                    &self.assignment,
                    &self.student
                ),
                7 => latex::dump_all_grade_sheets(&self.assignment),
                _ => break,
            }
        }
    }

    fn new_student(&mut self) {
        self.student = display::get_new_student_name();
        self.assignment.students.push(self.student.clone());
        self.student_idx = (self.assignment.students.len() as u32) - 1;
    }

    fn change_student(&mut self, dx: i32) {
        if dx >= 0 {
            let x = (self.student_idx + 1)
                % self.assignment.students.len() as u32;
            self.student_idx = x;
        } else if self.student_idx == 0 {
            self.student_idx = (self.assignment.students.len() as u32) - 1;
        } else {
            self.student_idx -= 1;
        }
        self.student = self.assignment.students[self.student_idx as usize].clone();
    }

    fn question_menu(&mut self) {
        let menu = vec![
            "Add New Comment".to_string(),
            "Add Existing Comment".to_string(),
            "Edit Comment".to_string(),
            "Remove Comment".to_string(),
            "Next Question".to_string(),
            "Prev Question".to_string(),
            "Display Question Comments".to_string(),
            "Quit".to_string()
        ];

        loop {
            // needs question marks and what it is out of
            let header = format!("Question {}.{}", self.question.num, self.question.part);
            let choice = display::get_menu_choice(&header, &menu);
            match choice {
                1 => self.add_new_comment(),
                2 => self.add_existing_comment(),
                3 => self.edit_comment(),
                4 => self.remove_comment(),
                5 => self.change_question(1),
                6 => self.change_question(-1),
                7 => display::question(
                    &self.assignment,
                    &self.student,
                    &self.question,
                ),
                _ => break,
            }
        }
    }

    fn add_new_comment(&mut self) {
        match display::new_comment() {
            Some((deduct, text)) => self.assignment.new_comment(
                &self.student,
                &self.question,
                deduct,
                text,
            ),
            _ => ()
        }
    }

    fn add_existing_comment(&mut self) {
        let com_id = display::choose_existing_comment(
            &self.assignment,
            &self.student,
            &self.question,
        );
        self.assignment.add_comment_to(
            &self.student,
            &self.question,
            com_id
        );
    }

    fn edit_comment(&mut self) {
        match display::edit_comment(
            &self.assignment,
            &self.student,
            &self.question,
        ) {
            Some((deduct, text, id)) => self.assignment.edit_comment(
                &self.question,
                id,
                deduct,
                text
            ),
            _ => ()
        };
    }

    fn remove_comment(&mut self) {
        match display::remove_comment(
            &self.assignment,
            &self.student,
            &self.question,
        ) {
            Some(id) => self.assignment.remove_comment_from(
                &self.student,
                &self.question,
                id
            ),
            _ => ()
        }
    }

    fn change_question(&mut self, dx: i32) {
        if dx >= 0 {
            let x = (self.question_idx + 1)
                % self.assignment.questions.len() as u32;
            self.question_idx = x;
        } else if self.question_idx == 0 {
            self.question_idx = (self.assignment.questions.len() as u32) - 1;
        } else {
            self.question_idx -= 1;
        }
        self.question = self.assignment.questions[self.question_idx as usize].clone();
    }
}

