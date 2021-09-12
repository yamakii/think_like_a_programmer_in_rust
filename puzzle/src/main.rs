#![allow(dead_code)]

fn main() {
    puzzle::triangle();
    digit::check_digit();
}

mod puzzle {
    use num::abs;
    pub fn triangle() {
        for i in 0..7 {
            for _ in 0..(4 - abs(4 - i)) {
                print!("#");
            }
            println!();
        }
    }
}

mod digit {
    use std::io;

    pub fn check_digit() {
        println!("Enter a numer");
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("error");
        let text = text.trim();

        let (even, odd): (Vec<_>, Vec<_>) = text
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .partition(|(i, _)| i % 2 == 0);
        let odd_length_checksum = sum_double(&even) + sum_single(&odd);
        let even_length_checksum = sum_single(&even) + sum_double(&odd);

        let checksum = if text.chars().count() % 2 == 0 {
            even_length_checksum
        } else {
            odd_length_checksum
        };

        println!("Checksum is {}.", checksum);
        if checksum % 10 == 0 {
            println!("Checksum is divisilbe by 10. Valid.");
        } else {
            println!("Checksum is not divisilbe by 10. Invalid.");
        }
    }

    fn sum_single(nums: &[(usize, u32)]) -> u32 {
        nums.iter().map(|(_i, n)| n).sum::<u32>()
    }

    fn sum_double(nums: &[(usize, u32)]) -> u32 {
        nums.iter()
            .map(|(_i, n)| double_digit_number(*n))
            .sum::<u32>()
    }

    fn double_digit_number(num: u32) -> u32 {
        let result = num * 2;
        if result < 10 {
            result
        } else {
            result % 10 + 1
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::digit::double_digit_number;
        #[test]
        fn test_double_digit_number() {
            assert_eq!(double_digit_number(4), 8);
            assert_eq!(double_digit_number(5), 1);
        }
    }
}
