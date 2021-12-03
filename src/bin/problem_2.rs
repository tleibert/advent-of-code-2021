//! Advent of code problem 2
//!
//! Written by Trevor Leibert

use std::fs::File;
use std::io::{BufRead, BufReader};

struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Position {
    fn new(horizontal: i64, depth: i64, aim: i64) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }

    fn update(&mut self, command: &str) {
        let mut parts = command.split_ascii_whitespace();
        let command = parts.next().unwrap();
        let quantity: i64 = parts.next().unwrap().parse().unwrap();

        match command {
            "up" => self.aim -= quantity,
            "down" => self.aim += quantity,
            _ => {
                self.horizontal += quantity;
                self.depth += self.aim * quantity
            }
        };
    }

    fn product(&self) -> i64 {
        self.horizontal * self.depth
    }
}

fn main() {
    let fh = File::open("inputs/input_2.txt").unwrap();
    let reader = BufReader::new(fh);

    let mut position = Position::new(0, 0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        position.update(&line);
    }

    println!("horizontal * depth {}", position.product());
}
