use crate::data::Assignment;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait FileOps {
    fn load_assignment(filename: &str) -> Assignment;
    fn save_assignment(assignment: &Assignment);
    fn save_latex_grade_sheet(assignment: &Assignment, student: &String);
    fn save_all_latex_grade_sheets(assignment: &Assignment);
}
