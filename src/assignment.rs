use crate::grade_sheet::GradeSheet;
use crate::comment::Comment;


#[derive(Debug)]
struct CommentAndCount(Comment, u32);

#[derive(Debug)]
pub struct Assignment<'a> {
    pub title: String,
    pub course: String,
    pub marks: Vec<Vec<u32>>,
    pub grade_sheets: Vec<GradeSheet<'a>>,
    comments: Vec<CommentAndCount>,
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

    pub fn new_comment(&mut self, deduction: u32, text: String) {
        self.comments.push(
            CommentAndCount(
                Comment::new(deduction, text),
                1,
            )
        )
    }

    pub fn get_comments(&self) -> Vec<&Comment> {
        self.comments
            .iter()
            .map(|c| &c.0)
            .collect()
    }

    pub fn com_count_inc() {}

    pub fn com_count_dec() {}

    pub fn num_comments(&self) -> usize {
        self.comments.len()
    }

    pub fn add_student() {}

    pub fn remove_student() {}
}


#[cfg(test)]
mod tests {
    use crate::assignment::Assignment;
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
        assert_eq!(0, asn.num_comments());
    }

    #[test]
    fn it_creates_a_new_comment() {
        let mut asn = Assignment::new(
            String::from("Assignment 1"),
            String::from("CPSC 3720"),
            vec![ vec![2, 4], vec![10]],
        );
        asn.new_comment(
            2,
            String::from("some comment text")
        );
        assert_eq!(1, asn.num_comments());
        let coms = asn.get_comments();
        assert_eq!(2, coms[0].deduction);
    }


}





