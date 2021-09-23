#![allow(dead_code)]

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
    use crate::*;
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
