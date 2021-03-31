use crate::data::{Assignment, Question};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait View {
    fn clear_screen();
    fn show_menu(header: &str, menu: &Vec<String>) -> u32;
    fn new_student(assignment: &Assignment) -> String;
    fn create_assignment() -> Assignment;
    fn load_assignment() -> Option<Assignment>;
    fn new_comment() -> Option<(f32, String)>;
    fn add_existing_comment(
        assignment: &Assignment,
        student: &str,
        question: &Question,
    ) -> Option<u64>;
    fn edit_comment(
        assignment: &Assignment,
        student: &str,
        question: &Question,
    ) -> Option<(f32, String, u64)>;
    fn remove_comment(assignment: &Assignment, student: &str, question: &Question) -> Option<u64>;
    fn show_grade_sheet(assignment: &Assignment, student: &str);
    fn show_question_info(assignment: &Assignment, student: &str, question: &Question);
}
