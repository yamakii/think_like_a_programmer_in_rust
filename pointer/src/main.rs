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
