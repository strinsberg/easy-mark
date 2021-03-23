use crate::comment::Comment;
use std::cmp;

#[derive(Debug)]
pub struct Grade {
    pub num: u32,
    pub part: u32,
    pub out_of: u32,
    pub comments: Vec<Comment>,
}

impl Grade {
    pub fn new(num: u32, part: u32, out_of: u32) -> Grade {
        Grade {
            num,
            part,
            out_of,
            comments: Vec::new(),
        }
    }

    pub fn get_mark(&self) -> u32 {
        cmp::max(
            0,
            self.out_of - self.comments.iter()
                                       .map(|c| c.deduction)
                                       .sum::<u32>()
        )

    }

    pub fn add_com(&mut self, com: &Comment) {
        self.comments.push((*com).clone());
    }

    pub fn del_com(&mut self, com: &Comment) {
        self.comments.retain(|c| c != com);
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
            2,
            String::from("some comment text")
        );
        let com2 = Comment::new(
            1,
            String::from("some other comment text")
        );
        g.add_com(&com);
        g.add_com(&com2);

        assert_eq!(1, g.get_mark());
    }
}
