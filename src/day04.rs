use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use multiset::HashMultiSet;

pub fn part1() {
    let input = std::fs::File::open("input/day04.txt").unwrap();
    let mut sum = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let (_, numbers) = line.split_once(':').unwrap();
        let (left, right) = numbers.split_once('|').unwrap();
        let winning: HashSet<&str> = left.split_whitespace().collect();
        let my: HashSet<&str> = right.split_whitespace().collect();
        let count = winning.intersection(&my).collect::<Vec<&&str>>().len();
        if count > 0 {
            sum += 2_i32.pow((count - 1) as u32);
        }
    }
    println!("{sum}");
}

pub fn part2() {
    let input = std::fs::File::open("input/day04.txt").unwrap();
    let mut cards = HashMultiSet::<usize>::new();
    let mut index = 0_usize;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let (_, numbers) = line.split_once(':').unwrap();
        let (left, right) = numbers.split_once('|').unwrap();
        let winning: HashSet<&str> = left.split_whitespace().collect();
        let my: HashSet<&str> = right.split_whitespace().collect();
        cards.insert(index);
        let cards_count = cards.count_of(&index);
        let numbers_count = winning.intersection(&my).collect::<Vec<&&str>>().len();
        for next_index in (index + 1)..=index + numbers_count {
            cards.insert_times(next_index, cards_count)
        }
        index += 1;
    }
    println!("{}", cards.len());
}
