mod comment;
mod grade;
mod assignment;

use comment::Comment;


fn main() {
    let com = Comment::new(
        String::from("djfsk-123"),
        2,
        String::from("some comment text")
    );

    println!("{}", com.to_text());
    println!("{}", com.to_latex());
}
