use crate::assignment::Assignment;
use crate::comment::Question;
use rustyline::Editor;
use std::fs;
use std::fs::File;
use std::process::Command;

pub fn clear_screen() {
    Command::new("clear").status().unwrap();
}

pub fn input(prompt: &str) -> String {
    readline_with_initial(prompt, ("", ""))
}

pub fn readline_with_initial(prompt: &str, initial: (&str, &str)) -> String {
    loop {
        let mut rl = Editor::<()>::new();
        let res: String = rl
            .readline_with_initial(prompt, initial)
            .unwrap()
            .trim()
            .to_string();
        if !res.is_empty() {
            return res;
        }
        println!("\n*** Input cannot be empty ***\n");
    }
}

pub fn get_u32(prompt: &str, error_msg: &str) -> u32 {
    loop {
        let num: String = input(prompt);
        match num.parse::<u32>() {
            Ok(x) => break x,
            _ => println!("\n*** {} ***\n", error_msg),
        }
    }
}

pub fn get_f32(prompt: &str, error_msg: &str) -> f32 {
    loop {
        let num: String = input(prompt);
        match num.parse::<f32>() {
            Ok(x) => break x,
            _ => println!("\n*** {} ***\n", error_msg),
        }
    }
}

pub fn get_menu_choice(header: &str, menu: &Vec<String>) -> u32 {
    println!("==== {} ====", header);
    for (i, item) in menu.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }

    let num = loop {
        match get_u32("Choice: ", "Input must be a positive number") {
            x if x != 0 && x <= menu.len() as u32 => break x,
            _ => println!("\n*** Choice must be from the menu ***\n"),
        }
    };
    clear_screen();
    num
}

pub fn get_new_student_name(assignment: &Assignment) -> String {
    print!("==== New Student ====\n");
    loop {
        match input("Student Name: ") {
            name if assignment.student_exists(&name) => {
                println!("\n*** A Student with that name has already been added ***\n")
            }
            name => {
                clear_screen();
                break name;
            }
        }
    }
}

pub fn create_new_assignment() -> Assignment {
    println!("==== New Assignment ====");
    let name = input("Assignment Name: ");
    let course = input("Course: ");
    let mut asn = Assignment::new(name, course);

    let num_q: u32 = loop {
        match get_u32(
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
            match get_u32(
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
                    asn.new_question(i, part_num, x);
                    part_num += 1;
                }
            }
        }
    }
    clear_screen();
    asn
}

pub fn load_assignment() -> Option<Assignment> {
    // This way of getting the file names is not the best, but it works
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
        let choice = (get_menu_choice(&header, &files) - 1) as usize;

        if choice >= files.len() - 1 {
            None
        } else {
            let f = File::open(&files[choice]).expect("Unable to create file");
            let asn: Assignment = serde_pickle::de::from_reader(f).expect("could not pickle");
            Some(asn)
        }
    } else {
        println!("*** No easy-mark files ***\n");
        None
    }
}

pub fn grade_sheet(assignment: &Assignment, student: &str) {
    println!("================================================================");
    println!("{} - {}", assignment.course, assignment.title);
    println!("{}", student);
    println!(
        "Total: {}/{}\n",
        assignment.total_mark(student),
        assignment.out_of()
    );
    for q in assignment.questions.iter() {
        question(assignment, student, &q);
    }
    println!("================================================================\n\n");
}

pub fn question(assignment: &Assignment, student: &str, question: &Question) {
    let comments = assignment.question_comments(student, question);
    let mark = assignment.question_mark(student, question);

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

pub fn new_comment() -> Option<(f32, String)> {
    println!("==== Add New Comment ====");
    let deduction: f32 = loop {
        match get_f32("Deduction: ", "Must be a whole or decimal number") {
            x if x < 0.0 => println!(
                "\n*** {}. {}. ***\n",
                "Deductions must be 0 or greater", "They will be negative when calculating marks"
            ),
            x => break x,
        }
    };
    let text: String = input("Comment: ");
    let satisfied: String = input("Satisfied? (y/n): ");
    clear_screen();

    match &satisfied.to_lowercase() == "y" {
        true => Some((deduction, text)),
        false => None,
    }
}

pub fn choose_existing_comment(
    assignment: &Assignment,
    student: &str,
    question: &Question,
) -> Option<u64> {
    let header = "Add Existing Comment";
    let comments = assignment.question_comments_unused(student, question);

    if comments.len() == 0 {
        clear_screen();
        println!("*** No available comments ***\n");
        return None;
    }

    let mut menu: Vec<String> = comments
        .iter()
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

pub fn edit_comment(
    assignment: &Assignment,
    student: &str,
    question: &Question,
) -> Option<(f32, String, u64)> {
    let header = "Edit Comment *** For ALL Users ***";
    let comments = assignment.question_comments(student, question);

    if comments.len() == 0 {
        clear_screen();
        println!("*** No comments have been added ***\n");
        return None;
    }

    let mut menu: Vec<String> = comments
        .iter()
        .map(|c| format!("[-{}]\n   {} ", c.deduction, c.text))
        .collect();
    menu.push("Cancel".to_string());

    let choice = (get_menu_choice(&header, &menu) - 1) as usize;
    clear_screen();

    if choice < comments.len() {
        println!("==== Edit Comment ====");
        let deduction: f32 = loop {
            let num: String =
                readline_with_initial("Deduction: ", (&comments[choice].deduction.to_string(), ""));
            match num.parse::<f32>() {
                Ok(x) => break x,
                _ => println!(
                    "\n*** {}. {}. ***\n",
                    "Deductions must be 0 or greater",
                    "They will be negative when calculating marks"
                ),
            }
        };

        let text: String = readline_with_initial("Comment: ", (&comments[choice].text, ""));
        let satisfied: String = input("Satisfied? (y/n): ");
        clear_screen();

        match satisfied.to_lowercase() == "y".to_string() {
            true => Some((deduction, text, comments[choice].id)),
            false => None,
        }
    } else {
        None
    }
}

pub fn remove_comment(assignment: &Assignment, student: &str, question: &Question) -> Option<u64> {
    let header = "Remove Comment".to_string();
    let comments = assignment.question_comments(student, question);

    if comments.len() == 0 {
        clear_screen();
        println!("*** No comments have been added ***\n");
        return None;
    }

    let mut menu: Vec<String> = comments
        .iter()
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
