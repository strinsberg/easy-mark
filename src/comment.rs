#[derive(Debug)]
pub struct Comment {
    pub deduction: u32,
    pub text: String,
}

impl Comment {
    pub fn new(deduction: u32, text: String) -> Comment {
        Comment { deduction, text }
    }
}

#[cfg(test)]
mod tests {
    use crate::comment::Comment;

    #[test]
    fn it_builds() {
        let com = Comment::new(
            2,
            String::from("some comment text")
        );
        assert_eq!(2, com.deduction);
        assert_eq!(String::from("some comment text"), com.text);
    }
} 

