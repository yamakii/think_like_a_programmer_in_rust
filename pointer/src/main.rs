#![allow(dead_code)]

fn main() {
    println!("hello");
}

mod array_string {
    type ArrayString = Box<[char]>;

    fn charactor_at(array_string: &ArrayString, position: usize) -> char {
        array_string[position]
    }

    fn lentgh(s: &ArrayString) -> usize {
        s.iter().take_while(|&&x| x != 0 as char).count()
    }

    fn append(array_string: &mut ArrayString, c: char) {
        let old_length = lentgh(array_string);
        let mut new_string: ArrayString = vec![0 as char; old_length + 2].into_boxed_slice();
        for i in 0..old_length {
            new_string[i] = array_string[i];
        }
        new_string[old_length] = c;
        *array_string = new_string;
    }

    fn concatenate(s1: &mut ArrayString, s2: &ArrayString) {
        let s1_old_length = lentgh(s1);
        let s2_length = lentgh(s2);
        let s1_new_length = s1_old_length + s2_length;
        let mut new_s: ArrayString = vec![0 as char; s1_new_length + 1].into_boxed_slice();
        for i in 0..s1_old_length {
            new_s[i] = s1[i];
        }
        for i in 0..s2_length {
            new_s[s1_old_length + i] = s2[i];
        }
        *s1 = new_s;
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_append() {
            let mut sut: ArrayString = Box::new(['t', 'e', 's', 't', 0 as char]);
            append(&mut sut, '!');
            assert_eq!(
                Box::new(['t', 'e', 's', 't', '!', 0 as char]) as ArrayString,
                sut,
            )
        }
        #[test]
        fn test_append_null() {
            let mut sut: ArrayString = Box::new([0 as char]);
            append(&mut sut, '!');
            assert_eq!(Box::new(['!', 0 as char]) as ArrayString, sut,)
        }
        #[test]
        fn test_concatenate() {
            let mut sut1: ArrayString = Box::new(['t', 'e', 's', 't', 0 as char]);
            let sut2: ArrayString = Box::new(['b', 'e', 'd', 0 as char]);
            concatenate(&mut sut1, &sut2);
            assert_eq!(
                Box::new(['t', 'e', 's', 't', 'b', 'e', 'd', 0 as char]) as ArrayString,
                sut1,
            )
        }
    }
}

mod linked_list_generics {
    struct Node<T> {
        elm: T,
        next: Link<T>,
    }
    type Link<T> = Option<Box<Node<T>>>;
    struct Student {
        number: u32,
        grade: u32,
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sample() {
            let mut node1 = Node {
                elm: Student {
                    number: 1001,
                    grade: 78,
                },
                next: None,
            };
            let mut node2 = Node {
                elm: Student {
                    number: 1012,
                    grade: 93,
                },
                next: None,
            };
            let node3 = Node {
                elm: Student {
                    number: 1078,
                    grade: 85,
                },
                next: None,
            };
            node2.next = Some(Box::new(node3));
            node1.next = Some(Box::new(node2));
            let student_collection = Some(Box::new(node1));
            print!("{:?}", student_collection.unwrap().elm.number);
        }
    }
}

mod linked_list {
    type StudentLink = Option<Box<StudentNode>>;
    struct StudentNode {
        number: u32,
        grade: u32,
        next: StudentLink,
    }
    type StudentCollection = Option<StudentNode>;

    fn add_record(student_collection: &mut StudentCollection, number: u32, grade: u32) {
        if let Some(current) = student_collection.take() {
            let head = StudentNode {
                number,
                grade,
                next: Some(Box::new(current)),
            };
            *student_collection = Some(head);
        }
    }

    fn average_record(student_collection: &StudentCollection) -> f64 {
        let mut count = 0;
        let mut sum = 0_f64;
        let mut next;
        if let Some(node) = student_collection {
            sum += node.grade as f64;
            count += 1;
            next = node.next.as_ref();
        } else {
            return 0_f64;
        };
        while let Some(node) = next {
            sum += node.grade as f64;
            count += 1;
            next = node.next.as_ref();
        }
        sum / count as f64
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        fn make_data() -> StudentCollection {
            let mut node1 = StudentNode {
                number: 1001,
                grade: 78,
                next: None,
            };
            let mut node2 = StudentNode {
                number: 1012,
                grade: 93,
                next: None,
            };
            let node3 = StudentNode {
                number: 1078,
                grade: 85,
                next: None,
            };
            node2.next = Some(Box::new(node3));
            node1.next = Some(Box::new(node2));
            Some(node1)
        }

        #[test]
        fn test_sample() {
            let student_collection = make_data();

            let node = student_collection.unwrap();
            assert_eq!(node.number, 1001);
            let node = node.next.unwrap();
            assert_eq!(node.number, 1012);
            let node = node.next.unwrap();
            assert_eq!(node.number, 1078);
            let node = node.next;
            assert!(node.is_none());
        }

        #[test]
        fn test_add_record() {
            let mut student_collection = make_data();
            add_record(&mut student_collection, 1274, 91);

            let node = student_collection.unwrap();
            assert_eq!(node.number, 1274);
        }

        #[test]
        fn test_average_record() {
            let student_collection = make_data();
            let avg = average_record(&student_collection);

            println!("{}", avg);
        }
    }
}
