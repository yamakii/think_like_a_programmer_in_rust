#![allow(dead_code)]

mod mode {
    use std::{cmp::Ordering, collections::HashMap};

    fn find_mode(answers: &[u32]) -> Option<u32> {
        let score = answers.iter().try_fold([9; 10], |mut acc, &a| {
            if a > acc.len() as u32 {
                None
            } else {
                acc[a as usize - 1] += 1;
                Some(acc)
            }
        });
        score.and_then(|x| {
            x.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| index as u32 + 1)
        })
    }

    fn find_mode_by_hash_map(answers: &[u32]) -> Option<u32> {
        let survay = answers.iter().try_fold(HashMap::new(), |mut acc, &a| {
            if a > 10 {
                None
            } else {
                let counter = acc.entry(a).or_insert(0);
                *counter += 1;
                Some(acc)
            }
        });
        survay.and_then(|x| {
            x.iter()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(&answer, _)| answer)
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_find_mode() {
            assert_eq!(Some(10), find_mode(&[1, 10, 10, 1, 10]));
            assert_eq!(None, find_mode(&[11, 10]));
        }
        #[test]
        fn test_find_mode_by_hash_map() {
            assert_eq!(Some(10), find_mode_by_hash_map(&[1, 10, 10, 1, 10]));
            assert_eq!(None, find_mode_by_hash_map(&[11]));
        }
        #[test]
        fn test_sort_student_struct() {
            struct Student {
                id: u32,
                grade: u32,
            }

            let mut students = [
                Student { id: 1, grade: 90 },
                Student { id: 2, grade: 60 },
                Student { id: 3, grade: 70 },
            ];

            students.sort_by(|a, b| a.id.cmp(&b.id));
            assert_eq!(&students[0].grade, &90);

            students.sort_by(|a, b| a.grade.cmp(&b.grade));
            assert_eq!(&students[0].id, &2);
        }
    }
}

mod sort {
    fn is_sorted(array: &[u32]) -> bool {
        let last = array[array.len() - 1];
        for each in array {
            if each > &last {
                return false;
            }
        }
        true
    }

    fn is_sorted1(array: &[u32]) -> bool {
        if array.len() < 2 {
            return true;
        }
        for i in 1..array.len() {
            if array[i - 1] > array[i] {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    mod tests {
        use crate::sort::*;

        #[test]
        fn test_sort_student_struct() {
            struct Student {
                id: u32,
                grade: u32,
            }

            let mut students = [
                Student { id: 1, grade: 90 },
                Student { id: 2, grade: 60 },
                Student { id: 3, grade: 70 },
            ];

            students.sort_by(|a, b| a.id.cmp(&b.id));
            assert_eq!(&students[0].grade, &90);

            students.sort_by(|a, b| a.grade.cmp(&b.grade));
            assert_eq!(&students[0].id, &2);
        }

        #[test]
        fn test_is_sorted() {
            assert!(is_sorted(&[1, 2, 3]));
            assert!(!is_sorted(&[1, 2, 3, 2]));

            assert!(is_sorted1(&[1, 2, 3]));
            assert!(!is_sorted1(&[1, 2, 3, 2]));
        }
    }
}

mod sales {
    const NUM_AGENTS: usize = 3;
    const NUM_MONTHS: usize = 12;

    fn sales() -> [[u32; NUM_MONTHS]; NUM_AGENTS] {
        [
            [
                1856, 598, 30924, 87478, 328, 2653, 387, 3754, 387587, 2873, 276, 32,
            ],
            [
                5865, 5456, 3983, 6464, 9957, 4785, 3875, 3838, 4959, 1122, 7766, 2534,
            ],
            [23, 55, 67, 00, 265, 376, 232, 223, 4546, 564, 4544, 3434],
        ]
    }

    fn average(array: &[u32]) -> f32 {
        let tottal: u32 = array.iter().sum();
        tottal as f32 / array.len() as f32
    }

    fn median(array: &mut [u32]) -> f32 {
        array.sort_unstable();
        match array.len() % 2_usize {
            0 => {
                let center = array.len() / 2;
                average(&array[center - 1..center + 1])
            }
            1 => {
                let center = (array.len() - 1) / 2;
                array[center] as f32
            }
            _ => unreachable!(),
        }
    }

    fn highest(array: &[f32]) -> &f32 {
        array
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_highest_average() {
            let sales = sales();
            let target = [average(&sales[0]), average(&sales[1]), average(&sales[2])];
            let highest = highest(&target);
            println!("highest !! {}", highest)
        }
        #[test]
        fn test_highest_median() {
            let mut sales = sales();
            let target = [
                median(&mut sales[0]),
                median(&mut sales[1]),
                median(&mut sales[2]),
            ];
            let highest = highest(&target);
            println!("highest !! {}", highest)
        }
    }
}
