/*
use crate::assignment::Assignment;
use crate::grade_sheet::GradeSheet;
use crate::grade::Grade;

pub struct App<'a> {
    pub assignment: Option<Assignment<'a>>,
    pub grade_sheet: Option<GradeSheet<'a>>,
    pub grade: Option<Grade<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            assignment: None,
            grade_sheet: None,
            grade: None,
        }
    }

    pub fn add_new_comment() {
        // should ask for deduction, text, and confirm
        // then make new_comment on assignment
        // borrow the comment and add it to the current grade
    }
    
    pub fn add_existing_comment() {
        // get a list of comments sorted by their counts
        // display the list and an option to take none
        // chosen comment gets count++ and gets added to the current grade
    }

    pub fn remove_comment() {
        // list comments in the current grade and option to cancel
        // chosen comment is removed from the current grade
        // in assignment list comment has its count reduced
        // if count is 0 then the comment should be removed from the assignment
    }

    pub fn edit_comment() {
        // list comments on the current grade and option to cancel
        // chose to update globally or locally
        // show the comment in question
        // give the comment prompt and input
        // if copy add new
        // else mutate the selected comment directly
    }
}


#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::assignment::Assignment;
    use crate::grade_sheet::GradeSheet;
    use crate::grade::Grade;

    fn setup_simple_assignment() {
        let mut asn = Assignment::new(
            String::from("test asn"),
            String::from("test course"),
            vec![ vec![2, 4], vec![10]],
        );
        asn.new_grade_sheet(String::from("Steven"));
        asn.new_grade_sheet(String::from("Josip"));

        asn.new_comment(
            1,
            String::from("comment on Q1.1"),
            asn.get_grade(&String::from("Steven"), 1, 1),
        );
        asn.new_comment(
            2,
            String::from("comment on Q1.2"),
            asn.get_grade(&String::from("Steven"), 1, 2),
        );
        asn.new_comment(
            5,
            String::from("comment on Q2.1"),
            asn.get_grade(&String::from("Steven"), 2, 1),
        );
        // new comment should require a grade, so it can be added directly


    }

    #[test]
    fn it_builds() {


    }
}
*/
