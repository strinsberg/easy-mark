use crate::assignment::Assignment;
use crate::comment::Question;
use read_input::prelude::*;
use std::process::Command;

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
    println!("================================================================");
    println!("{} - {}", assignment.course, assignment.title);
    println!("{}", student);
    println!("Total: {}/{}\n", assignment.student_mark(student), assignment.out_of());
    for q in assignment.questions.iter() {
        println!("----------------------------------------------------------------");
        question(assignment, student, &q);
    }
    println!("================================================================\n\n");
}

pub fn question(assignment: &Assignment, student: &String, question: &Question) {
    let comments = assignment.question_comments(student, question);
    let mark = assignment.question_mark(student, question);

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

pub fn new_comment() -> Option<(u32, String)> {
    println!("==== Add New Comment ====");
    let deduction: u32 = loop {
         let num: String = input().msg("Deduction: ").get();
         match num.parse::<u32>() {
             Ok(x) if x >= 0 => break x,
             _ => println!("\n*** Enter 0 or higher for the deduction ***\n"),
         }
    };

    let text: String = input().msg("Comment: ").get();
    let satisfied: String = input().msg("Satisfied? (y/n): ").get();
    clear_screen();

    match satisfied.to_lowercase() == "y".to_string() {
        true => Some((deduction, text)),
        false => None
    }
}

pub fn choose_existing_comment(assignment: &Assignment, student: &String, question: &Question) -> Option<u64> {
    let header = "Add Existing Comment".to_string();
    let comments = assignment.question_comments_unused(student, question);

    if comments.len() == 0 {
        clear_screen();
        println!("*** No unused comments ***\n");
        return None;
    }

    let mut menu: Vec<String> = comments.iter()
        .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
        .collect();
    menu.push("Cancel".to_string());

    let choice = (get_menu_choice(&header, &menu) - 1) as usize;
    clear_screen();

    if choice < comments.len() {
        Some(comments[choice].id)
    } else {
        None
    }
}

pub fn edit_comment(assignment: &Assignment, student: &String, question: &Question) -> Option<(u32, String, u64)> {
    let header = "Edit Comment".to_string();
    let comments = assignment.question_comments(student, question);

    if comments.len() == 0 {
        clear_screen();
        println!("*** No comments added yet ***\n");
        return None;
    }

    let mut menu: Vec<String> = comments.iter()
        .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
        .collect();
    menu.push("Cancel".to_string());

    let choice = (get_menu_choice(&header, &menu) - 1) as usize;
    clear_screen();

    if choice < comments.len() {
        println!("==== Existing Comment ====");
        println!("Deduction: {}", comments[choice].deduction);
        println!("Text: {}\n", comments[choice].text);

        match new_comment() {
            Some((deduct, text)) => Some((deduct, text, comments[choice].id)),
            _ => None
        }
    } else {
        None
    }
}

pub fn remove_comment(assignment: &Assignment, student: &String, question: &Question) -> Option<u64> {
    let header = "Remove Comment".to_string();
    let comments = assignment.question_comments(student, question);
    
    if comments.len() == 0 {
        clear_screen();
        println!("*** No comments added yet ***\n");
        return None;
    }

    let mut menu: Vec<String> = comments.iter()
        .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
        .collect();
    menu.push("Cancel".to_string());

    let choice = (get_menu_choice(&header, &menu) - 1) as usize;
    clear_screen();

    if choice < comments.len() {
        Some(comments[choice].id)
    } else {
        None
    }
}
