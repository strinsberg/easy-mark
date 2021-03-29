use crate::data::{Assignment, Comment};

pub fn convert_assignment(assignment: &Assignment, student: &String) -> String {
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
            assignment.students_total(student),
            assignment.out_of()
        ),
        "\\date{\\today}".to_string(),
        "\\begin{document}".to_string(),
        "\\maketitle".to_string(),
        convert_questions(assignment, student),
        "\\end{document}".to_string(),
    ]
    .join("\n")
}

fn convert_questions(assignment: &Assignment, student: &String) -> String {
    assignment
        .get_questions()
        .iter()
        .fold(String::new(), |acc, q| {
            acc + "\n"
                + &format!(
                    "\\section*{{{}.{} -- {}/{}}}\n",
                    q.num,
                    q.part,
                    assignment.students_mark_for(student, &q),
                    q.out_of
                )
                + &match assignment
                    .students_comments_for(student, &q)
                    .iter()
                    .fold(String::new(), |acc, c| acc + "\n" + &convert_comment(&c))
                {
                    s if s.is_empty() => "Well Done".to_string(),
                    s => "\\begin{description}".to_string() + &s + "\n" + "\\end{description}",
                }
        })
}

pub fn convert_comment(comment: &Comment) -> String {
    if comment.deduction > 0.0 {
        format!(
            "\\item[\\color{{red}}-{}] {}",
            comment.deduction, comment.text
        )
    } else {
        format!("\\item[Note] {}", comment.text)
    }
}
