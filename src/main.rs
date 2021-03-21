pub trait Writer {
    fn to_text(&self) -> String;
    fn to_latex(&self) -> String;
}

#[derive(Debug)]
struct Comment {
    id: String,
    deduction: u32,
    text: String,
}

impl Comment {
    fn build(id: String, deduction: u32, text: String) -> Comment {
        Comment { id, deduction, text }
    }
}

impl Writer for Comment {
    fn to_text(&self) -> String {
        format!("[-{}]\n   {}", self.deduction, self.text)
    }

    fn to_latex(&self) -> String {
        format!("[-{}] \\\\ {}", self.deduction, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;  // not appropriate once moved to another file
    #[test]
    fn it_builds() {
        let com = Comment::build(
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
        let com = Comment::build(
            String::from("com-123"),
            2,
            String::from("some comment text")
        );
        assert_eq!(String::from("[-2]\n   some comment text"), com.to_text());
    }

    #[test]
    fn it_writes_to_latex() {
        let com = Comment::build(
            String::from("com-123"),
            2,
            String::from("some comment text")
        );
        assert_eq!(String::from("[-2] \\\\ some comment text"), com.to_latex());
    }
} 



fn main() {
    let com = Comment::build(
        String::from("djfsk-123"),
        2,
        String::from("some comment text")
    );

    println!("{}", com.to_text());
    println!("{}", com.to_latex());
}
