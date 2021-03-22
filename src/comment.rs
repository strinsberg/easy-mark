#[derive(Debug)]
pub struct Comment {
    pub id: String,
    pub deduction: u32,
    pub text: String,
}

impl Comment {
    pub fn new(id: String, deduction: u32, text: String) -> Comment {
        Comment { id, deduction, text }
    }

    pub fn to_text(&self) -> String {
        format!("[-{}]\n   {}", self.deduction, self.text)
    }

    pub fn to_latex(&self) -> String {
        format!("[-{}] \\\\ {}", self.deduction, self.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::comment::Comment;

    #[test]
    fn it_builds() {
        let com = Comment::new(
            String::from("com-123"),
            2,
            String::from("some comment text")
        );
        assert_eq!(String::from("com-123"), com.id);
        assert_eq!(2, com.deduction);
        assert_eq!(String::from("some comment text"), com.text);
    }

    #[test]
    fn it_writes_to_text() {
        let com = Comment::new(
            String::from("com-123"),
            2,
            String::from("some comment text")
        );
        assert_eq!(String::from("[-2]\n   some comment text"), com.to_text());
    }

    #[test]
    fn it_writes_to_latex() {
        let com = Comment::new(
            String::from("com-123"),
            2,
            String::from("some comment text")
        );
        assert_eq!(String::from("[-2] \\\\ some comment text"), com.to_latex());
    }
} 

