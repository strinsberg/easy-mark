use crate::assignment::Assignment;
use crate::comment::Question;
use read_input::prelude::*;

pub fn get_menu_choice(header: &String, menu: &Vec<String>) -> u32 {
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

pub fn get_new_student_name() -> String {
    print!("==== New Student ====\n");
    input().msg("Student Name: ").get()
}

pub fn create_new_assignment() -> Assignment {
    print!("==== New Assignment ====\n");
    // get assignment name
    // get assignment course
    // get number of questions
    // get marks for each question part
    // return the new assignment
    Assignment::new(
        "default".to_string(),
        "none".to_string()
    )
}

pub fn load_assignment() -> Option<Assignment> {
    print!("\nLoad Asn\n\n");
    // get list of valid files
    // show a menu for them
    // load the chosen one
    // return option, if none then show menu again
    Some(Assignment::new(
        "default".to_string(),
        "none".to_string()
    ))
}

pub fn grade_sheet(assignment: &Assignment, student: &String) {
    // show assignment details and student name
    // get the total assignment grade, and student marks
    // for each question get all comments for the student
    // find the total marks and display all comments
    // order biggest mistakes to notes (consider better way)
    print!("\nDisplay sheet\n\n");
}

pub fn question(assignment: &Assignment, student: &String, question: &Question) {
    // get all comments for this question and student
    // show question num, part, and marks/total
    // show each comment
    print!("\nDisplay Question\n\n");
}

// should be an option if they decide to discard it
pub fn new_comment() -> (u32, String) {
    print!("\nAdd New Comment\n\n");
    // show header
    // get deduction and text
    (0, "none".to_string())
}

// should return option if they dont pick a comment
pub fn choose_existing_comment(assignment: &Assignment, student: &String, question: &Question) -> u64 {
    print!("\nAdd Existing\n\n");
    // get all comments for a question that do not contain student
    // menu choice for them
    // return the comment id
    0
}

// should return option if they dont pick a comment, or they discard changes
pub fn edit_comment(assignment: &Assignment, student: &String, question: &Question) -> (u32, String, u64) {
    print!("\nEdit Comment\n\n");
    // list all the comments that we have to choose from
    // display the chosen one
    // take info for the new comment
    // return the deduction and text
    (0, "none".to_string(), 0)
}

// should return option if they dont pick a comment, or they discard changes
pub fn remove_comment(assignment: &Assignment, student: &String, question: &Question) -> u64 {
    print!("\nRemove Comment\n\n");
    // list all the comments that we have to choose from
    // return the id
    0
}
