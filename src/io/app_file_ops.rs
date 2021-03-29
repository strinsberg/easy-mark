use crate::data::{latex, Assignment};
use crate::io::FileOps;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct AppFileOps;

impl FileOps for AppFileOps {
    fn load_assignment(filename: &str) -> Assignment {
        let f = File::open(filename).expect("Unable to open file");
        serde_pickle::de::from_reader(f).expect("could not pickle")
    }

    fn save_assignment(assignment: &Assignment) {
        let filename =
            format!("{}_{}.emark", assignment.course, assignment.title).replace(" ", "_");
        let mut f = File::create(filename).expect("Unable to create file");
        serde_pickle::ser::to_writer(&mut f, assignment, true).expect("could not pickle");
    }

    fn save_latex_grade_sheet(assignment: &Assignment, student: &String) {
        let filename = format!(
            "{}_{}.tex",
            student.replace(" ", "_"),
            assignment
                .students_total(student)
                .to_string()
                .replace(".", "_")
        );
        let latex = latex::convert_assignment(assignment, student);
        let mut f = File::create(filename).expect("Could not open file for writing");
        f.write_all(latex.as_bytes())
            .expect("Could not write to file");
    }

    fn save_all_latex_grade_sheets(assignment: &Assignment) {
        let dirname =
            format!("./{}_{}_latex", assignment.course, assignment.title).replace(" ", "_");
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
                    assignment.students_total(&s).to_string().replace(".", "_")
                );
            let latex = latex::convert_assignment(assignment, &s);
            let mut f = File::create(filename).expect("Could not open file for writing");
            f.write_all(latex.as_bytes())
                .expect("Could not write to file");
            println!("{}", s);
        }
        println!("");
    }
}
