use std::io::{BufRead, BufReader};
use std::ops::Not;

pub fn part1() {
    let input = std::fs::File::open("input/day01.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut sum: u32 = 0;
    for line in lines {
        let mut first_digit = ' ';
        let mut last_digit = ' ';
        for character in line.unwrap().chars() {
            if character.is_digit(10) {
                if first_digit == ' ' {
                    first_digit = character
                }
                last_digit = character
            }
        }
        let mut number = String::new();
        number.push(first_digit);
        number.push(last_digit);
        sum += number.parse::<u32>().unwrap();
    }
    println!("{}", sum);
}

pub fn part2() {
    let input = std::fs::File::open("input/day01.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut sum: u32 = 0;
    for line in lines {
        let mut line = line.unwrap();
        let mut first_digit = ' ';
        let mut last_digit = ' ';
        while line.is_empty().not() {
            let digit: char;
            let char = line.chars().next().unwrap();
            if char.is_digit(10) {
                digit = char;
                line.remove(0);
            } else if line.starts_with("one") {
                digit = '1';
                line = line.strip_prefix("one").unwrap().to_string();
            } else if line.starts_with("two") {
                digit = '2';
                line = line.strip_prefix("two").unwrap().to_string();
            } else if line.starts_with("three") {
                digit = '3';
                line = line.strip_prefix("three").unwrap().to_string();
            } else if line.starts_with("four") {
                digit = '4';
                line = line.strip_prefix("four").unwrap().to_string();
            } else if line.starts_with("five") {
                digit = '5';
                line = line.strip_prefix("five").unwrap().to_string();
            } else if line.starts_with("six") {
                digit = '6';
                line = line.strip_prefix("six").unwrap().to_string();
            } else if line.starts_with("seven") {
                digit = '7';
                line = line.strip_prefix("seven").unwrap().to_string();
            } else if line.starts_with("eight") {
                digit = '8';
                line = line.strip_prefix("eight").unwrap().to_string();
            } else if line.starts_with("nine") {
                digit = '9';
                line = line.strip_prefix("nine").unwrap().to_string();
            } else {
                digit = ' ';
                line.remove(0);
            }
            if digit != ' ' {
                if first_digit == ' ' {
                    first_digit = digit;
                }
                last_digit = digit;
            }
        }
        let mut number = String::new();
        number.push(first_digit);
        number.push(last_digit);
        sum += number.parse::<u32>().unwrap();
    }
    println!("{}", sum);
}
