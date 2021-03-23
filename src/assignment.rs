/*
use crate::grade_sheet::GradeSheet;
use crate::grade::Grade;
use crate::comment::Comment;


#[derive(Debug)]
pub struct CommentAndCount{ com: Comment, count: u32 }

#[derive(Debug)]
pub struct Assignment<'a> {
    pub title: String,
    pub course: String,
    pub marks: Vec<Vec<u32>>,
    pub grade_sheets: Vec<GradeSheet<'a>>,
    pub comments: Vec<CommentAndCount>,
}

impl<'a> Assignment<'a> {
    pub fn new(title: String, course: String, marks: Vec<Vec<u32>>) -> Assignment<'a> {
        Assignment {
            title,
            course,
            marks,
            grade_sheets: Vec::new(),
            comments: Vec::new(),
        }
    }

    pub fn new_comment(&mut self, deduction: u32, text: String, grade: &mut Grade<'a>) {
        self.comments.push(
            CommentAndCount {
                com: Comment::new(deduction, text),
                count: 1,
            }
        );
    }

    pub fn new_grade_sheet(&mut self, student: String) {
        self.grade_sheets.push(
            GradeSheet::new(
                student,
                &self.marks,
            )
        )
    }

    pub fn get_grade(&mut self, student: &String, q_num: u32, q_part: u32) -> &mut Grade<'a> {
        self.grade_sheets
            .iter_mut()
            .find(|gs| gs.student == *student)
            .unwrap() // fix later with match and return option
            .grades
            .iter_mut()
            .find(|g| g.num == q_num && g.part == q_part)
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::assignment::Assignment;
    use crate::grade::Grade;
    use crate::comment::Comment;

    #[test]
    fn it_builds() {
        let asn = Assignment::new(
            String::from("Assignment 1"),
            String::from("CPSC 3720"),
            vec![ vec![2, 4], vec![10]],
        );
        assert_eq!(String::from("Assignment 1"), asn.title);
        assert_eq!(String::from("CPSC 3720"), asn.course);
        assert_eq!(2, asn.marks[0][0]);
        assert_eq!(10, asn.marks[1][0]);
        assert_eq!(0, asn.comments.len());
    }

    #[test]
    fn it_creates_a_new_comment() {
        let mut asn = Assignment::new(
            String::from("Assignment 1"),
            String::from("CPSC 3720"),
            vec![ vec![2, 4], vec![10]],
        );
        let mut g = Grade::new(1,3,4);
        asn.new_comment(
            2,
            String::from("some comment text"),
            &mut g,
        );
        assert_eq!(1, asn.comments.len());
        assert_eq!(1, asn.comments[0].count);
        assert_eq!(2, asn.comments[0].com.deduction);
        assert_eq!("some comment text", asn.comments[0].com.text);
    }

    #[test]
    fn it_creates_a_() {
        let mut asn = Assignment::new(
            String::from("Assignment 1"),
            String::from("CPSC 3720"),
            vec![ vec![2, 4], vec![10]],
        );
        asn.new_grade_sheet(String::from("Steven"));
        assert_eq!(1, asn.grade_sheets.len());
        assert_eq!(String::from("Steven"), asn.grade_sheets[0].student);
        assert_eq!(16, asn.grade_sheets[0].total);
    }
}




*/
