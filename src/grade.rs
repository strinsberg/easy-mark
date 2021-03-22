use crate::comment::Comment;
use std::cmp;

#[derive(Debug)]
pub struct Grade<'a> {
    pub num: u32,
    pub part: u32,
    pub out_of: u32,
    pub comments: Vec<&'a Comment>,
}

impl<'a> Grade<'a> {
    pub fn new(num: u32, part: u32, out_of: u32) -> Grade<'a> {
        Grade {
            num,
            part,
            out_of,
            comments: Vec::<&'a Comment>::new(),
        }
    }

    pub fn get_mark(&self) -> u32 {
        cmp::max(
            0,
            self.out_of - self.comments.iter()
                                       .fold(0, |acc, com| acc + com.deduction)
        )

    }

    pub fn to_text(&self) -> String {
        String::from("nil")
    }

    pub fn to_latex(&self) -> String {
        String::from("nil")
    }
}


#[cfg(test)]
mod tests {
    use crate::grade::Grade;
    use crate::comment::Comment;

    #[test]
    fn it_builds() {
        let g = Grade::new(1,3,4);
        assert_eq!(1, g.num);
        assert_eq!(3, g.part);
        assert_eq!(4, g.out_of);
        assert_eq!(0, g.comments.len());
    }

    #[test]
    fn it_gets_the_total_mark() {
        let mut g = Grade::new(1,3,4);
        let com = Comment::new(
            String::from("com-123"),
            2,
            String::from("some comment text")
        );
        let com2 = Comment::new(
            String::from("com-456"),
            1,
            String::from("some other comment text")
        );
        g.comments.push(&com);
        g.comments.push(&com2);

        assert_eq!(1, g.get_mark());
    }
}
