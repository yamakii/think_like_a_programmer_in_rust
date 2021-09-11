#![allow(dead_code)]

use num::abs;
use std::io;

fn main() {
    // triangle();
    check_digit();
}

fn triangle() {
    for i in 0..7 {
        for _ in 0..(4 - abs(4 - i)) {
            print!("#");
        }
        println!();
    }
}

fn check_digit() {
    println!("Enter a numer");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("error");
    let text = text.trim();

    let mut odd_length_checksum = 0;
    let mut even_length_checksum = 0;
    for (i, c) in text.chars().enumerate() {
        let num = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            odd_length_checksum += double_digit_number(num);
            even_length_checksum += num;
        } else {
            odd_length_checksum += num;
            even_length_checksum += double_digit_number(num);
        }
    }

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
    use crate::double_digit_number;
    #[test]
    fn test_double_digit_number() {
        assert_eq!(double_digit_number(4), 8);
        assert_eq!(double_digit_number(5), 1);
    }
}
