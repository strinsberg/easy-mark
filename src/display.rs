use crate::assignment::Assignment;
use crate::comment::Question;
use read_input::prelude::*;
use std::process::Command;
use std::cmp;

pub fn clear_screen() {
    Command::new("clear").status().unwrap();
}

pub fn get_menu_choice(header: &String, menu: &Vec<String>) -> u32 {
    let num = loop {
        println!("==== {} ====", header);
        for (i, item) in menu.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }

        let choice: String = input().msg("Choice: ").get();
        match choice.parse::<u32>() {
            Ok(x) if x != 0 && x <= menu.len() as u32 => break x,
            _ => println!("\n*** Invalid Choice ***\n"),
        }
    };
    clear_screen();
    num
}

pub fn get_new_student_name() -> String {
    print!("==== New Student ====\n");
    let name = input().msg("Student Name: ").get();
    clear_screen();
    name
}

pub fn create_new_assignment() -> Assignment {
    println!("==== New Assignment ====");
    let name = input().msg("Assignment Name: ").get();
    let course = input().msg("Course: ").get();
    let mut asn = Assignment::new(name, course);

    let num_q: u32 = loop {
        let choice: String = input().msg("Number of Questions: ").get();
        match choice.parse::<u32>() {
            Ok(x) => break x,
            _ => println!("\n*** Please choose a number ***\n"),
        }
    };

    for i in 1..(num_q+1) {
        println!("\n==== Marks for Question {} (0 to finish) ====", i);
        let mut part_num = 1;
        loop {
            let choice: String = input().msg(format!("Marks for {}.{}: ", i, part_num)).get();
            match choice.parse::<u32>() {
                Ok(x) if x <= 0 => {
                    if part_num > 1 {
                        break
                    } else {
                        println!("\n*** Each Question Must have at least 1 part ***\n");
                    }
                },
                Ok(x) => {
                    asn.new_question(i, part_num, x);
                    part_num += 1;
                },
                _ => println!("\n*** Please choose a number ***\n"),
            }
        }
    }
    clear_screen();
    asn
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
    let comments = assignment.comments
        .get(question)
        .unwrap()
        .iter()
        .filter(|c| c.names.contains(student));

    let mark = question.out_of as i32
        - comments.clone().fold(0, |acc, c| acc + c.deduction) as i32;

    println!("Question {}.{}", question.num, question.part);
    println!("Grade {}/{}\n", cmp::max(mark, 0), question.out_of);
    for com in comments {
        println!("{} -- {}", com.deduction, com.text);
    }
    println!("");
}

// should be an option if they decide to discard it
pub fn new_comment() -> (u32, String) {
    println!("==== Add New Comment ====");
    let deduction: u32 = loop {
         let num: String = input().msg("Deduction: ").get();
         match num.parse::<u32>() {
             Ok(x) if x >= 0 => break x,
             _ => println!("\n*** Enter 0 or higher for the deduction ***\n"),
         }
    };
    let text = input().msg("Comment: ").get();
    clear_screen();
    (deduction, text)
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