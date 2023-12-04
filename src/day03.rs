use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let input = std::fs::File::open("input/day03.txt").unwrap();
    let mut schematic = read_schematic(input);

    for row in 1..schematic.len() - 1 {
        for column in 1..schematic[row].len() - 1 {
            match schematic[row][column].char {
                digit if digit.is_digit(10) => {},
                '.' => {},
                _ => {
                    for move_row in 0..3 {
                        for move_column in 0..3 {
                            schematic[row - 1 + move_row][column - 1 + move_column].adjacent_to_symbol = true;
                        }
                    };
                }
            }
        }
    }

    let mut digits = String::new();
    let mut adjacent_to_symbol= false;
    let mut sum = 0;
    for row in 1..schematic.len() - 1 {
        for column in 1..schematic[row].len() {
            let node = &schematic[row][column];
            let is_digit = node.char.is_digit(10);
            if is_digit {
                // We're parsing a number right now!
                digits.push(node.char);
                if node.adjacent_to_symbol {
                    // If this digit is adjacent to a symbol, mark the whole number as adjacent.
                    adjacent_to_symbol = true;
                }
            } else {
                // We're not parsing a number.
                // In case we just advanced from the last digit we need to finish parsing the number
                // and reset some things.
                if !digits.is_empty() && adjacent_to_symbol {
                    sum += digits.parse::<i32>().unwrap();
                }
                digits.clear();
                adjacent_to_symbol = false;
            }
        }
    }
    println!("{sum}");
}

pub fn part2() {
    let input = std::fs::File::open("input/day03.txt").unwrap();
    let mut schematic = read_schematic(input);

    let mut gear_count: usize = 0;
    for row in 1..schematic.len() - 1 {
        for column in 1..schematic[row].len() - 1 {
            if schematic[row][column].char == '*' {
                for move_row in 0..3 {
                    for move_column in 0..3 {
                        schematic[row - 1 + move_row][column - 1 + move_column].gears.insert(gear_count);
                    };
                };
                gear_count += 1;
            };
        }
    }
    let mut gears: Vec<Vec<usize>> = Vec::with_capacity(gear_count);
    for _i in 0..gear_count {
        gears.push(Vec::new());
    }

    let mut digits = String::new();
    let mut adjacent_gears: HashSet<usize> = HashSet::new();
    for row in 1..schematic.len() - 1 {
        for column in 1..schematic[row].len() {
            let node = &schematic[row][column];
            let is_digit = node.char.is_digit(10);
            if is_digit {
                // We're parsing a number right now!
                digits.push(node.char);
                adjacent_gears.extend(&node.gears);
            } else {
                // We're not parsing a number.
                // In case we just advanced from the last digit we need to finish parsing the number
                // and reset some things.
                if !digits.is_empty() {
                    let number = digits.parse::<usize>().unwrap();
                    for gear in &adjacent_gears {
                        gears[*gear].push(number);
                    }
                }
                digits.clear();
                adjacent_gears.clear();
            }
        }
    }

    let mut sum = 0;
    for gear in gears {
        if gear.len() != 2 {
            continue
        }
        sum += gear[0] * gear[1];
    }
    println!("{sum}");
}

fn read_schematic(input: File) -> Vec<Vec<Node>> {
    let extra_node = Node::new('.');
    let mut schematic: Vec<Vec<Node>> = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut row: Vec<Node> = Vec::new();
        // To avoid checking bounds later, push an extra fake column at the start…
        row.push(extra_node.clone());
        for char in line.chars() {
            row.push(Node::new(char));
        }
        // …and at the end…
        row.push(extra_node.clone());
        schematic.push(row);
    }
    // …and push extra rows at the top and bottom.
    let mut extra = Vec::with_capacity(schematic[0].len());
    extra.resize(extra.len(), extra_node);
    schematic.insert(0, extra.clone());
    schematic.push(extra.clone());
    schematic
}

#[derive(Clone)]
struct Node {
    char: char,
    adjacent_to_symbol: bool,
    gears: HashSet<usize>,
}

impl Node {
    fn new(char: char) -> Self {
        Node {
            char,
            adjacent_to_symbol: false,
            gears: HashSet::new(),
        }
    }
}
