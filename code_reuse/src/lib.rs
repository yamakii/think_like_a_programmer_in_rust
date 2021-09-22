#![allow(dead_code)]

mod student_list {
    use std::error::Error;

    #[derive(Debug, Default, PartialEq, Clone)]
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

    #[derive(Debug, Default, PartialEq, Eq, Clone, PartialOrd)]
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

    #[derive(Debug, Default)]
    struct StudentCollection {
        head: StudentLink,
        policy: FirstStudentPolicy,
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    struct StudentNode {
        student_data: StudentRecord,
        next: StudentLink,
    }

    type StudentLink = Option<Box<StudentNode>>;

    #[derive(Debug)]
    enum FirstStudentPolicy {
        HierGrade,
        LowerStudentNumber,
        NameComesFirst,
    }

    impl Default for FirstStudentPolicy {
        fn default() -> Self {
            Self::NameComesFirst
        }
    }

    impl FirstStudentPolicy {
        fn higher_priority_than(&self, a1: &StudentRecord, a2: &StudentRecord) -> bool {
            match self {
                FirstStudentPolicy::HierGrade => a1.grade > a2.grade,
                FirstStudentPolicy::LowerStudentNumber => a1.student_id < a2.student_id,
                FirstStudentPolicy::NameComesFirst => a1.name < a2.name,
            }
        }
    }

    impl StudentCollection {
        fn add_record(&mut self, student: StudentRecord) {
            let head = if let Some(current) = self.head.take() {
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
            self.head = Some(Box::new(head));
        }

        fn record_with_number(&self, number: &u32) -> Option<&StudentRecord> {
            let mut current = &self.head;
            while let Some(node) = current {
                if &node.student_data.student_id == number {
                    return Some(&node.student_data);
                }
                current = &node.next;
            }
            None
        }

        fn remove_record(&mut self, number: &u32) {
            let mut target = &mut self.head;
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

        fn set_first_student_policy(&mut self, policy: FirstStudentPolicy) {
            self.policy = policy;
        }

        fn first_student(&self) -> Option<&StudentRecord> {
            if let Some(head_node) = &self.head {
                let mut first_node = head_node;
                let mut current = Some(head_node);
                while let Some(node) = current {
                    if self
                        .policy
                        .higher_priority_than(&node.student_data, &first_node.student_data)
                    {
                        first_node = node;
                    }
                    current = if let Some(next) = &node.next {
                        Some(next)
                    } else {
                        None
                    };
                }
                Some(&first_node.student_data)
            } else {
                None
            }
        }

        fn first_item_iterator(&self) -> StudentCollectionIterator {
            let initial = if let Some(node) = &self.head {
                Some(node)
            } else {
                None
            };
            StudentCollectionIterator::new(initial)
        }
    }

    #[derive(Debug, Default)]
    struct StudentCollectionIterator<'a> {
        #[allow(clippy::borrowed_box)]
        current: Option<&'a Box<StudentNode>>,
    }

    impl<'a> StudentCollectionIterator<'a> {
        #[allow(clippy::borrowed_box)]
        fn new(initial: Option<&'a Box<StudentNode>>) -> Self {
            Self { current: initial }
        }

        fn advance(&mut self) {
            if let Some(node) = self.current {
                self.current = if let Some(next_node) = &node.next {
                    Some(next_node)
                } else {
                    None
                };
            }
        }

        fn student(&self) -> Option<&StudentRecord> {
            if let Some(node) = &self.current {
                Some(&node.student_data)
            } else {
                None
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
            assert_eq!(sut.head, Some(Box::new(node1.clone())));

            let record2 = StudentRecord::new(85, 1034, "Yuko".to_string()).unwrap();
            sut.add_record(record2.clone());
            let node2 = StudentNode {
                student_data: record2,
                next: Some(Box::new(node1)),
            };
            assert_eq!(sut.head, Some(Box::new(node2)));
        }

        #[test]
        fn test_record_with_number() {
            let mut sut = StudentCollection::default();
            assert_eq!(sut.record_with_number(&1001), None);

            let record1 = StudentRecord::new(73, 1023, "Hiroshi".to_string()).unwrap();
            sut.add_record(record1.clone());
            assert_eq!(sut.record_with_number(&1023), Some(&record1));

            assert_eq!(sut.record_with_number(&1024), None);
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

            sut.remove_record(&1045);
            assert_eq!(sut.record_with_number(&1045), None);
        }

        #[test]
        fn test_policy() {
            let mut sut = StudentCollection::default();

            let record1 = StudentRecord::new(63, 1023, "Hiroshi".to_string()).unwrap();
            let record2 = StudentRecord::new(75, 1034, "Yuko".to_string()).unwrap();
            let record3 = StudentRecord::new(84, 1045, "Keita".to_string()).unwrap();
            let record4 = StudentRecord::new(83, 1016, "Naoto".to_string()).unwrap();
            sut.add_record(record1.clone());
            sut.add_record(record2);
            sut.add_record(record3.clone());
            sut.add_record(record4.clone());

            sut.set_first_student_policy(FirstStudentPolicy::HierGrade);
            assert_eq!(sut.first_student(), Some(&record3));
            sut.set_first_student_policy(FirstStudentPolicy::LowerStudentNumber);
            assert_eq!(sut.first_student(), Some(&record4));
            sut.set_first_student_policy(FirstStudentPolicy::NameComesFirst);
            assert_eq!(sut.first_student(), Some(&record1));
        }

        #[test]
        fn test_iterator() {
            let mut sut = StudentCollection::default();
            let record1 = StudentRecord::new(63, 1023, "Hiroshi".to_string()).unwrap();
            let record2 = StudentRecord::new(75, 1034, "Yuko".to_string()).unwrap();
            let record3 = StudentRecord::new(82, 1045, "Keita".to_string()).unwrap();
            let record4 = StudentRecord::new(81, 1056, "Naoto".to_string()).unwrap();
            sut.add_record(record1);
            sut.add_record(record2);
            sut.add_record(record3);
            sut.add_record(record4);

            let mut total = 0;
            let mut iter = sut.first_item_iterator();
            while let Some(node) = iter.student() {
                total += node.grade.0;
                iter.advance();
            }

            assert_eq!(total, 301);
        }
    }
}
