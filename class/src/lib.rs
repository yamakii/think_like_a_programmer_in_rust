#![allow(dead_code)]

mod student_list {
    use std::error::Error;

    #[derive(Debug, Default)]
    struct StudentRecord {
        grade: Grade,
        student_id: u32,
        name: String,
    }

    impl StudentRecord {
        fn new(grade: u32, student_id: u32, name: String) -> Result<Self, Box<dyn Error>> {
            Ok(Self {
                grade: Grade::new(grade)?,
                student_id,
                name,
            })
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Grade(u32);

    #[derive(Debug)]
    enum StudentError {
        GradeError(u32),
    }

    impl std::fmt::Display for StudentError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use self::StudentError::*;
            match self {
                GradeError(g) => write!(f, "GradeError {}", g),
            }
        }
    }

    impl Error for StudentError {}

    impl Grade {
        const LETTERS: [(u32, &'static str); 12] = [
            (0, "F"),
            (60, "D"),
            (67, "D+"),
            (70, "C-"),
            (73, "C"),
            (77, "C+"),
            (80, "B-"),
            (83, "B"),
            (87, "B+"),
            (90, "A-"),
            (93, "A"),
            (97, "A+"),
        ];

        fn new(grade: u32) -> Result<Grade, Box<dyn Error>> {
            if grade > 100 {
                Err(Box::new(StudentError::GradeError(grade)))
            } else {
                Ok(Grade(grade))
            }
        }

        fn letter(&self) -> &'static str {
            Self::LETTERS
                .iter()
                .find(|(score, _)| score >= &self.0)
                .unwrap_or(&Self::LETTERS[0])
                .1
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_grade() {
            let grade = Grade::new(93).unwrap();
            assert_eq!(&Grade(93), &grade);
            assert_eq!(&"A", &grade.letter());
        }
    }
}
