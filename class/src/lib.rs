#![allow(dead_code)]

mod student_list {
    use std::{error::Error, marker::PhantomData};

    #[derive(Debug, Default, PartialEq, Clone)]
    struct StudentRecord {
        grade: Grade,
        student_id: Number<StudentRecord>,
        name: Name<StudentRecord>,
    }

    impl StudentRecord {
        fn new(grade: u32, student_id: u32, name: String) -> Result<Self, Box<dyn Error>> {
            Ok(Self {
                grade: Grade::new(grade)?,
                student_id: Number::new(student_id),
                name: Name::new(name),
            })
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    struct Grade(u32);

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    struct Number<T>(u32, PhantomData<T>);

    impl<T> Number<T> {
        fn new(number: u32) -> Self {
            Number(number, PhantomData)
        }
        fn value(&self) -> &u32 {
            &self.0
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    struct Name<T>(String, PhantomData<T>);

    impl<T> Name<T> {
        fn new(name: String) -> Self {
            Name(name, PhantomData)
        }
        fn value(&self) -> &str {
            &self.0
        }
    }

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

    #[derive(Debug, Default, PartialEq)]
    struct StudentCollection(StudentLink);

    #[derive(Debug, Default, PartialEq, Clone)]
    struct StudentNode {
        student_data: StudentRecord,
        next: StudentLink,
    }

    type StudentLink = Option<Box<StudentNode>>;

    impl StudentCollection {
        fn add_record(&mut self, student: StudentRecord) {
            let head = if let Some(current) = self.0.take() {
                StudentNode {
                    student_data: student,
                    next: Some(current),
                }
            } else {
                StudentNode {
                    student_data: student,
                    next: None,
                }
            };
            self.0 = Some(Box::new(head));
        }

        fn record_with_number(&self, number: &Number<StudentRecord>) -> Option<&StudentRecord> {
            let mut current = &self.0;
            while let Some(node) = current {
                if &node.student_data.student_id == number {
                    return Some(&node.student_data);
                }
                current = &node.next;
            }
            None
        }

        fn remove_record(&mut self, number: &Number<StudentRecord>) {
            let mut target = &mut self.0;
            // 各要素を変更する可能性があるので、takeで取り出し、変更可能にする
            while let Some(mut node) = target.take() {
                if &node.student_data.student_id == number {
                    // 生徒番号が一致する場合は自身のnextを削除して、nextの指すオブジェクトに入れ替える
                    *target = node.next.take();
                    break;
                } else {
                    // 一致しない場合は変更せずに戻す
                    *target = Some(node);
                };
                // 走査対象を次へ
                if let Some(node) = target {
                    target = &mut node.next;
                }
            }
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

        #[test]
        fn test_add() {
            let mut sut = StudentCollection::default();

            let record1 = StudentRecord::new(73, 1023, "Hiroshi".to_string()).unwrap();
            sut.add_record(record1.clone());
            let node1 = StudentNode {
                student_data: record1,
                next: None,
            };
            assert_eq!(sut.0, Some(Box::new(node1.clone())));

            let record2 = StudentRecord::new(85, 1034, "Yuko".to_string()).unwrap();
            sut.add_record(record2.clone());
            let node2 = StudentNode {
                student_data: record2,
                next: Some(Box::new(node1)),
            };
            assert_eq!(sut.0, Some(Box::new(node2)));
        }

        #[test]
        fn test_record_with_number() {
            let mut sut = StudentCollection::default();
            assert_eq!(sut.record_with_number(&Number::new(1001)), None);

            let record1 = StudentRecord::new(73, 1023, "Hiroshi".to_string()).unwrap();
            sut.add_record(record1.clone());
            assert_eq!(sut.record_with_number(&Number::new(1023)), Some(&record1));

            assert_eq!(sut.record_with_number(&Number::new(1024)), None);
        }

        #[test]
        fn test_remove() {
            let mut sut = StudentCollection::default();

            let record1 = StudentRecord::new(63, 1023, "Hiroshi".to_string()).unwrap();
            let record2 = StudentRecord::new(75, 1034, "Yuko".to_string()).unwrap();
            let record3 = StudentRecord::new(82, 1045, "Keita".to_string()).unwrap();
            let record4 = StudentRecord::new(82, 1056, "Naoto".to_string()).unwrap();
            sut.add_record(record1);
            sut.add_record(record2);
            sut.add_record(record3);
            sut.add_record(record4);

            sut.remove_record(&Number::new(1045));
            assert_eq!(sut.record_with_number(&Number::new(1045)), None);
        }
    }
}
