use crate::grade::Grade;
use std::convert::TryInto;

#[derive(Debug)]
pub struct GradeSheet<'a> {
    pub student: String,
    pub total: u32,
    pub grades: Vec<Grade<'a>>,
}

impl<'a> GradeSheet<'a> {
    pub fn new(student: String, marks: &Vec<Vec<u32>>) -> GradeSheet<'a> {
        let mut grades = Vec::new();
        let mut total = 0;
        for (i, v) in marks.iter().enumerate() {
            for (j, m) in v.iter().enumerate() {
                grades.push(
                    Grade::new(
                        (i + 1).try_into().unwrap(),
                        (j + 1).try_into().unwrap(),
                        *m
                    )
                );
                total += *m;
            }
        }

        GradeSheet {
            student,
            total,
            grades,
        }
    }

    pub fn get_mark(&self) -> u32 {
        self.grades
            .iter()
            .map(|g| g.get_mark())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::grade_sheet::GradeSheet;
    use crate::grade::Grade;

    #[test]
    fn it_builds() {
        let marks = vec![ vec![2, 4], vec![10]];
        let asn = GradeSheet::new(String::from("Steve"), &marks);

        assert_eq!(String::from("Steve"), asn.student);
        assert_eq!(16, asn.total);
        assert_eq!(3, asn.grades.len());

        assert_eq!(2, asn.grades[0].out_of);
        assert_eq!(1, asn.grades[0].num);
        assert_eq!(1, asn.grades[0].part);

        assert_eq!(4, asn.grades[1].out_of);
        assert_eq!(1, asn.grades[1].num);
        assert_eq!(2, asn.grades[1].part);

        assert_eq!(10, asn.grades[2].out_of);
        assert_eq!(2, asn.grades[2].num);
        assert_eq!(1, asn.grades[2].part);
    }

    #[test]
    fn it_gets_the_correct_marks() {
        let marks = vec![ vec![2, 4], vec![10]];
        let asn = GradeSheet::new(String::from("Steve"), &marks);

        assert_eq!(16, asn.get_mark());
    } 
}

