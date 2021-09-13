#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap};

fn find_mode(answers: &[u32]) -> Option<u32> {
    let mut score = [0; 10];
    for answer in answers {
        if score.len() < *answer as usize {
            return None;
        }
        score[(*answer - 1) as usize] += 1;
    }
    score
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index as u32 + 1)
}

fn find_mode_by_hash_map(answers: &[u32]) -> Option<u32> {
    let mut survay: HashMap<&u32, u32> = HashMap::new();
    for answer in answers {
        if *answer > 10 {
            return None;
        }
        if let Some(a) = survay.get_mut(answer) {
            *a += 1;
        } else {
            survay.insert(answer, 1);
        }
    }
    survay
        .iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(answer, _)| **answer)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_find_mode() {
        assert_eq!(Some(10), find_mode(&[1, 10, 10, 1, 10]));
        assert_eq!(None, find_mode(&[11]));
    }
    #[test]
    fn test_find_mode_by_hash_map() {
        assert_eq!(Some(10), find_mode_by_hash_map(&[1, 10, 10, 1, 10]));
        assert_eq!(None, find_mode_by_hash_map(&[11]));
    }
}
