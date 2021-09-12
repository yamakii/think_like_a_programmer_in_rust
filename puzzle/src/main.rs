#![allow(dead_code)]

fn main() {
    // puzzle::triangle();
    // digit::check_digit();
    message::print_digit();
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
        let odd_length_checksum = || sum_double(&even) + sum_single(&odd);
        let even_length_checksum = || sum_single(&even) + sum_double(&odd);

        let checksum = if text.chars().count() % 2 == 0 {
            even_length_checksum()
        } else {
            odd_length_checksum()
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

mod message {
    use std::io;

    struct ModeType {
        next: Mode,
        base_number: u8,
        conversion_func: fn(u8) -> char,
    }
    #[derive(Debug)]
    enum Mode {
        UpperCase,
        LowerCase,
        Punctuation,
    }
    impl Mode {
        fn mode_type(&self) -> ModeType {
            match self {
                Self::UpperCase => ModeType {
                    next: Self::LowerCase,
                    base_number: 27,
                    conversion_func: |d: u8| to_upper_alphabet(d),
                },
                Self::LowerCase => ModeType {
                    next: Self::Punctuation,
                    base_number: 27,
                    conversion_func: |d: u8| to_lower_alphabet(d),
                },
                Self::Punctuation => ModeType {
                    next: Self::UpperCase,
                    base_number: 9,
                    conversion_func: |d: u8| to_symbol(d),
                },
            }
        }
        fn target_number(&self, source: u32) -> u8 {
            (source % self.mode_type().base_number as u32) as u8
        }
        fn convert(&self, d: u8) -> char {
            (self.mode_type().conversion_func)(d)
        }
        fn next(&self) -> Self {
            self.mode_type().next
        }
    }

    pub fn print_digit() {
        let mut mode = Mode::UpperCase;
        println!("Enter a numer text");
        let text = input_chars();
        for d in list_digits(&text) {
            println!("That number as an integer: {}", d);

            let target = mode.target_number(d);
            println!("That number as an target: {}", target);

            if target == 0 {
                mode = mode.next();
                println!("Change!: {:?}", mode);
            } else {
                let output = mode.convert(target);
                println!("Convert!: {}", output);
            }
        }
    }

    fn list_digits(text: &str) -> Vec<u32> {
        let mut chars = text.chars();
        let mut result = Vec::new();
        while let Some(c) = chars.next() {
            let mut overall_number = to_digit(c);
            for c in &mut chars {
                if c == ',' {
                    break;
                }
                let digit = to_digit(c);
                overall_number = overall_number * 10 + digit;
            }
            result.push(overall_number);
        }
        result
    }

    fn input_chars() -> String {
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("error");
        text.trim().to_string()
    }

    fn to_digit(c: char) -> u32 {
        c as u32 - b'0' as u32
    }

    fn to_lower_alphabet(d: u8) -> char {
        (d + b'a' - 1) as char
    }

    fn to_upper_alphabet(d: u8) -> char {
        (d + b'A' - 1) as char
    }

    fn to_symbol(d: u8) -> char {
        match d {
            1 => '!',
            2 => '?',
            3 => ',',
            4 => '.',
            5 => ' ',
            6 => ';',
            7 => '"',
            8 => '\'',
            _ => 0 as char,
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::message::*;
        #[test]
        fn test_list_digits() {
            assert_eq!(vec![123, 456, 789], list_digits("123,456,789"))
        }
        #[test]
        fn test_to_digit() {
            assert_eq!(8, to_digit('8'))
        }
        #[test]
        fn test_to_lower_alphabet() {
            assert_eq!('c', to_lower_alphabet(3))
        }
        #[test]
        fn test_to_upper_alphabet() {
            assert_eq!('C', to_upper_alphabet(3))
        }
    }
}
