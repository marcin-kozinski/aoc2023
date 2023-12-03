use std::cmp;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let input = std::fs::File::open("input/day02.txt").unwrap();
    let mut game = 0;
    let mut sum = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        game += 1;
        let (_, record) = line.split_once(": ").unwrap();
        let subsets: Vec<&str> = record.split("; ").collect();
        let mut possible = true;
        for subset in subsets {
            let cubes_by_color: Vec<&str> = subset.split(", ").collect();
            for cubes in cubes_by_color {
                let (number, color) = cubes.split_once(' ').unwrap();
                let number = number.parse::<i32>().unwrap();
                match color {
                    "red" if number > 12 => possible = false,
                    "green" if number > 13 => possible = false,
                    "blue" if number > 14 => possible = false,
                    _ => ()
                }
            }
        }
        if possible {
            sum += game;
        }
    }
    println!("{sum}");
}

pub fn part2() {
    let input = std::fs::File::open("input/day02.txt").unwrap();
    let mut sum = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let (_, record) = line.split_once(": ").unwrap();
        let subsets: Vec<&str> = record.split("; ").collect();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for subset in subsets {
            let cubes_by_color: Vec<&str> = subset.split(", ").collect();
            for cubes in cubes_by_color {
                let (number, color) = cubes.split_once(' ').unwrap();
                let number: u32 = number.parse().unwrap();
                match color {
                    "red" => max_red = cmp::max(max_red, number),
                    "green" => max_green = cmp::max(max_green, number),
                    "blue" => max_blue = cmp::max(max_blue, number),
                    _ => ()
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    println!("{sum}");
}
