use crate::data::{Assignment, Comment};
use std::fs;
use std::fs::File;
use std::io::Write;

// Should probably just get this to make the directory and filename
// so that it always puts the file into the sub directory
pub fn dump_grade_sheet(assignment: &Assignment, student: &String) {
    let filename = format!(
        "{}_{}.tex",
        student.replace(" ", "_"),
        assignment.total_mark(student).to_string().replace(".", "_")
    );
    write_grade_sheet_to(assignment, student, &filename);
    println!("*** Successfully Wrote Grade Sheet To Latex ***\n");
}

pub fn write_grade_sheet_to(assignment: &Assignment, student: &String, filename: &String) {
    let latex = assignment_to_latex(assignment, student);
    let mut f = File::create(filename).expect("Could not open file for writing");
    f.write_all(latex.as_bytes())
        .expect("Could not write to file");
}

pub fn assignment_to_latex(assignment: &Assignment, student: &String) -> String {
    let empty = "".to_string();
    let questions = assignment
        .get_questions()
        .iter()
        .fold(String::new(), |acc, q| {
            acc + "\n"
                + &format!(
                    "\\section*{{{}.{} -- {}/{}}}\n",
                    q.num,
                    q.part,
                    assignment.question_mark(student, &q),
                    q.out_of
                )
                + &match assignment
                    .question_comments(student, &q)
                    .iter()
                    .fold(String::new(), |acc, c| acc + "\n" + &comment_to_latex(&c))
                {
                    s if s == empty => "Well Done".to_string(),
                    s => "\\begin{description}".to_string() + &s + "\n" + "\\end{description}",
                }
        });

    vec![
        "\\documentclass{article}".to_string(),
        "\\usepackage{fullpage}".to_string(),
        "\\usepackage{xcolor}".to_string(),
        format!(
            "\\title{{{} {} Grading}}",
            assignment.course, assignment.title
        ),
        format!(
            "\\author{{{} \\\\ \\textbf{{Score: {}/{}}} }}",
            student,
            assignment.total_mark(student),
            assignment.out_of()
        ),
        "\\date{\\today}".to_string(),
        "\\begin{document}".to_string(),
        "\\maketitle".to_string(),
        questions,
        "\\end{document}".to_string(),
    ]
    .join("\n")
}

pub fn comment_to_latex(comment: &Comment) -> String {
    if comment.deduction > 0.0 {
        format!(
            "\\item[\\color{{red}}-{}] {}",
            comment.deduction, comment.text
        )
    } else {
        format!("\\item[Note] {}", comment.text)
    }
}

pub fn dump_all_grade_sheets(assignment: &Assignment) {
    let dirname = format!("./{}_{}_latex", assignment.course, assignment.title).replace(" ", "_");
    if !fs::metadata(&dirname).is_ok() {
        fs::create_dir(&dirname).expect("Error creating directory for latex files");
    }

    println!("==== Writing All Grade Sheets To Latex ====");
    for s in assignment.get_students().iter() {
        let filename = dirname.clone()
            + "/"
            + &format!(
                "{}_{}.tex",
                s.replace(" ", "_"),
                assignment.total_mark(&s).to_string().replace(".", "_")
            );
        write_grade_sheet_to(assignment, &s, &filename);
        println!("{}", s);
    }
    println!("");
}
