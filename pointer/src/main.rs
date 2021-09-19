#![allow(dead_code)]

fn main() {
    println!("hello");
}

type ArrayString = Box<[char]>;

fn charactor_at(array_string: &ArrayString, position: usize) -> char {
    array_string[position]
}

fn append(array_string: &mut ArrayString, c: char) {
    let mut old_length = 0;
    while array_string[old_length] != 0 as char {
        old_length += 1;
    }
    let mut new_string: ArrayString = vec![0 as char; old_length + 2].into_boxed_slice();
    for i in 0..old_length {
        new_string[i] = array_string[i];
    }
    new_string[old_length] = c;
    *array_string = new_string;
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
}
