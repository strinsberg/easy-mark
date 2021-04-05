use crate::data::{Assignment, Question};
use crate::io::{input, AppFileOps, FileOps, View};
use std::fs;
use std::process::Command;

#[cfg(target_os = "linux")]
static CLEAR: &str = "clear";
#[cfg(target_os = "windows")]
static CLEAR: &str = "cls";

pub struct AppView;

impl View for AppView {
    fn clear_screen() {
        Command::new(CLEAR).status().unwrap();
    }

    fn show_menu(header: &str, menu: &Vec<String>) -> u32 {
        println!("==== {} ====", header);
        for (i, item) in menu.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }

        let num = loop {
            match input::get_u32("Choice: ", "Input must be a positive number") {
                x if x != 0 && x <= menu.len() as u32 => break x,
                _ => println!("\n*** Choice must be from the menu ***\n"),
            }
        };
        Self::clear_screen();
        num
    }

    fn new_student(assignment: &Assignment) -> String {
        print!("==== New Student ====\n");
        loop {
            match input::get_line("Student Name: ") {
                name if assignment.student_exists(&name) => {
                    println!("\n*** A Student with that name has already been added ***\n")
                }
                name => {
                    Self::clear_screen();
                    break name;
                }
            }
        }
    }

    fn create_assignment() -> Assignment {
        println!("==== New Assignment ====");
        let name = input::get_line("Assignment Name: ");
        let course = input::get_line("Course: ");
        let mut asn = Assignment::new(name, course);

        let num_q: u32 = loop {
            match input::get_u32(
                "Number of Questions: ",
                "Input must be a positive whole number",
            ) {
                x if x > 0 => break x,
                _ => println!("\n*** Must have at least one question ***\n"),
            }
        };

        for i in 1..(num_q + 1) {
            println!("\n==== Marks for Question {} (0 to finish) ====", i);
            let mut part_num = 1;
            loop {
                match input::get_u32(
                    &format!("Marks for {}.{}: ", i, part_num),
                    "Input must be a positive number",
                ) {
                    x if x <= 0 => {
                        if part_num > 1 {
                            break;
                        } else {
                            println!("\n*** Must have at least 1 part ***\n");
                        }
                    }
                    x => {
                        asn.add_question(i, part_num, x);
                        part_num += 1;
                    }
                }
            }
        }
        Self::clear_screen();
        asn
    }

    fn load_assignment() -> Option<Assignment> {
        let mut files: Vec<String> = fs::read_dir("./")
            .unwrap()
            .map(|p| {
                p.unwrap()
                    .path()
                    .display()
                    .to_string()
                    .strip_prefix("./")
                    .unwrap()
                    .to_string()
            })
            .filter(|s| s.ends_with(".emark"))
            .collect();

        if files.len() > 0 {
            files.push("Exit".to_string());
            let header = "Load Assignment";
            let choice = (Self::show_menu(&header, &files) - 1) as usize;

            if choice >= files.len() - 1 {
                None
            } else {
                let asn = AppFileOps::load_assignment(&files[choice]);
                Some(asn)
            }
        } else {
            println!("*** No easy-mark files ***\n");
            None
        }
    }

    fn new_comment() -> Option<(f32, String)> {
        println!("==== Add New Comment ====");
        let deduction: f32 = loop {
            match input::get_f32("Deduction: ", "Must be a whole or decimal number") {
                x if x < 0.0 => println!(
                    "\n*** {}. {}. ***\n",
                    "Deductions must be 0 or greater",
                    "They will be negative when calculating marks"
                ),
                x => break x,
            }
        };
        let text: String = input::get_line("Comment: ");
        let satisfied: String = input::get_line("Satisfied? (y/n): ");
        Self::clear_screen();

        match &satisfied.to_lowercase() == "y" {
            true => Some((deduction, text)),
            false => None,
        }
    }

    fn add_existing_comment(
        assignment: &Assignment,
        student: &str,
        question: &Question,
    ) -> Option<u64> {
        let header = "Add Existing Comment";
        let comments = assignment.unused_comments_for(student, question);

        if comments.len() == 0 {
            Self::clear_screen();
            println!("*** No available comments ***\n");
            return None;
        }

        let mut menu: Vec<String> = comments
            .iter()
            .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
            .collect();
        menu.push("Cancel".to_string());

        let choice = (Self::show_menu(&header, &menu) - 1) as usize;
        Self::clear_screen();

        if choice < comments.len() {
            Some(comments[choice].id)
        } else {
            None
        }
    }

    fn edit_comment(
        assignment: &Assignment,
        student: &str,
        question: &Question,
    ) -> Option<(f32, String, u64)> {
        let header = "Edit Comment *** For ALL Users ***";
        let comments = assignment.students_comments_for(student, question);

        if comments.len() == 0 {
            Self::clear_screen();
            println!("*** No comments have been added ***\n");
            return None;
        }

        let mut menu: Vec<String> = comments
            .iter()
            .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
            .collect();
        menu.push("Cancel".to_string());

        let choice = (Self::show_menu(&header, &menu) - 1) as usize;
        Self::clear_screen();

        if choice < comments.len() {
            println!("==== Edit Comment ====");
            let deduction: f32 = loop {
                let num: String = input::readline_with_initial(
                    "Deduction: ",
                    (&comments[choice].deduction.to_string(), ""),
                );
                match num.parse::<f32>() {
                    Ok(x) => break x,
                    _ => println!(
                        "\n*** {}. {}. ***\n",
                        "Deductions must be 0 or greater",
                        "They will be negative when calculating marks"
                    ),
                }
            };

            let text: String =
                input::readline_with_initial("Comment: ", (&comments[choice].text, ""));
            let satisfied: String = input::get_line("Satisfied? (y/n): ");
            Self::clear_screen();

            match satisfied.to_lowercase() == "y".to_string() {
                true => Some((deduction, text, comments[choice].id)),
                false => None,
            }
        } else {
            None
        }
    }

    fn remove_comment(assignment: &Assignment, student: &str, question: &Question) -> Option<u64> {
        let header = "Remove Comment".to_string();
        let comments = assignment.students_comments_for(student, question);

        if comments.len() == 0 {
            Self::clear_screen();
            println!("*** No comments have been added ***\n");
            return None;
        }

        let mut menu: Vec<String> = comments
            .iter()
            .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
            .collect();
        menu.push("Cancel".to_string());

        let choice = (Self::show_menu(&header, &menu) - 1) as usize;
        Self::clear_screen();

        if choice < comments.len() {
            Some(comments[choice].id)
        } else {
            None
        }
    }

    fn show_grade_sheet(assignment: &Assignment, student: &str) {
        println!("================================================================");
        println!("{} - {}", assignment.course, assignment.title);
        println!("{}", student);
        println!(
            "Total: {}/{}\n",
            assignment.students_total(student),
            assignment.out_of()
        );
        for q in assignment.get_questions().iter() {
            Self::show_question_info(assignment, student, &q);
        }
        println!("================================================================\n\n");
    }

    fn show_question_info(assignment: &Assignment, student: &str, question: &Question) {
        let comments = assignment.students_comments_for(student, question);
        let mark = assignment.students_mark_for(student, question);

        println!("--------------------------------------");
        println!("Question {}.{}", question.num, question.part);
        println!("Grade {}/{}\n", mark, question.out_of);

        if comments.len() == 0 {
            println!("** Well Done **");
        } else {
            for com in comments {
                println!("[-{}]\n   {}", com.deduction, com.text);
            }
        }
        println!("");
    }
}
