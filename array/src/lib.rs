#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap};

fn find_mode(answers: &[u32]) -> usize {
    let mut score = [0; 10];
    for answer in answers {
        if score.len() < *answer as usize {
            return 0;
        }
        score[(*answer - 1) as usize] += 1;
    }
    if let Some(index) = score
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
    {
        index + 1_usize
    } else {
        0
    }
}

fn find_mode_by_hash_map(answers: &[u32]) -> u32 {
    let mut survay: HashMap<&u32, u32> = HashMap::new();
    for answer in answers {
        if *answer > 10 {
            return 0;
        }
        if let Some(a) = survay.get_mut(answer) {
            *a += 1;
        } else {
            survay.insert(answer, 1);
        }
    }
    if let Some(answer) = survay
        .iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(answer, _)| **answer)
    {
        answer
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_find_mode() {
        assert_eq!(10, find_mode(&[1, 10, 10, 1, 10]));
        assert_eq!(0, find_mode(&[11]));
    }
    #[test]
    fn test_find_mode_by_hash_map() {
        assert_eq!(10, find_mode_by_hash_map(&[1, 10, 10, 1, 10]));
        assert_eq!(0, find_mode_by_hash_map(&[11]));
    }
}
