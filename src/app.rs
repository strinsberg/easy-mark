use crate::assignment::Assignment;
use crate::comment::Comment;
use crate::comment::Question;
use read_input::prelude::*;

pub struct App {
    assignment: Assignment,
    student: String,
    student_num: u32,
    question: Question,
}

impl App {
    pub fn new() -> App {
        App {
            assignment: Assignment::new(
                "default".to_string(),
                "none".to_string()
            ),
            student: "none".to_string(),
            student_num: 0,
            question: Question {
                num: 0,
                part: 0,
                out_of: 0
            },
        }
    }

    pub fn run(&mut self) {
        let header = "Main Menu".to_string();
        let menu = vec![
            "New Assignment".to_string(),
            "Load Assignment".to_string(),
            "Quit".to_string()
        ];

        let choice = App::get_menu_choice(&header, &menu);
        match choice {
            1 => self.assignment = App::new_assignment(),
            2 => self.assignment = App::load_assignment(),
            _ => return ()
        }
        self.asn_menu();
    }

    fn get_menu_choice(header: &String, menu: &Vec<String>) -> u32 {
        loop {
            print!("==== {} ====\n", header);
            for (i, item) in menu.iter().enumerate() {
                print!("{}. {}\n", i + 1, item);
            }

            let choice: String = input().msg("Choice: ").get();
            match choice.parse::<u32>() {
                Ok(x) if x != 0 && x <= menu.len() as u32 => return x,
                _ => print!("\n*** Invalid Choice ***\n\n"),
            }
        }
    }

    fn new_assignment() -> Assignment {
        // try to put logic in separate testable function
        print!("\nNew Asn\n\n");
        Assignment::new(
            "default".to_string(),
            "none".to_string()
        )
    }

    fn load_assignment() -> Assignment {
        // try to put logic in separate testable function
        print!("\nLoad Asn\n\n");
        Assignment::new(
            "default".to_string(),
            "none".to_string()
        )
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
                self.assignment.course,
                self.student
            );

            let choice = App::get_menu_choice(&header, &menu);
            match choice {
                1 => self.question_menu(),
                2 => self.new_student(),
                3 => self.change_student(1),
                4 => self.change_student(-1),
                5 => self.display_grade_sheet(),
                6 => self.grade_sheet_to_latex(),
                7 => self.all_to_latex(),
                _ => break,
            }
        }
    }

    fn new_student(&mut self) {
        print!("==== New Student ====\n");
        let name = input().msg("Student Name: ").get();
        // this part could be split out and tested
        self.student = name;
        self.assignment.students.push(self.student.clone());
        self.student_num = (self.assignment.students.len() as u32) - 1;
    }

    fn change_student(&mut self, dx: i32) {
        // this could be tested
        if dx >= 0 {
            let x = (self.student_num + 1)
                % self.assignment.students.len() as u32;
            self.student_num = x;
        } else if self.student_num == 0 {
            self.student_num = (self.assignment.students.len() as u32) - 1;
        } else {
            self.student_num -= 1;
        }
        self.student = self.assignment.students[self.student_num as usize].clone();
    }

    fn display_grade_sheet(&mut self) {
        print!("\nDisplay sheet\n\n");
    }

    fn grade_sheet_to_latex(&mut self) {
        print!("\nSheet to latex\n\n");
    }

    fn all_to_latex(&mut self) {
        print!("\nAll to latex\n\n");
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
            let header = "Grading Menu".to_string();  // add Qnum and grade
            let choice = App::get_menu_choice(&header, &menu);
            match choice {
                1 => self.add_new_comment(),
                2 => self.add_existing_comment(),
                3 => self.edit_comment(),
                4 => self.remove_comment(),
                5 => self.change_question(1),
                6 => self.change_question(-1),
                7 => self.display_question(),
                _ => break,
            }
        }
    }

    fn add_new_comment(&mut self) {
        print!("\nAdd New\n\n");

    }

    fn add_existing_comment(&mut self) {
        print!("\nAdd Existing\n\n");

    }

    fn edit_comment(&mut self) {
        print!("\nEdit Comment\n\n");

    }

    fn remove_comment(&mut self) {
        print!("\nRemove Comment\n\n");

    }

    fn change_question(&mut self, dx: i32) {
        print!("\nChange Question\n\n");

    }

    fn display_question(&mut self) {
        print!("\nDisplay Question\n\n");
    }
}

