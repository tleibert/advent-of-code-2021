//! Advent of code problem 2
//!
//! Written by Trevor Leibert

use std::fs::File;
use std::io::{BufRead, BufReader};

struct Position {
    horizontal: i32,
    depth: i32,
}

fn main() {
    let fh = File::open("inputs/input_2.txt").unwrap();
    let reader = BufReader::new(fh);

    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for line in reader.lines() {
        let line = line.unwrap();
        update_position(&mut position, &line);
    }

    println!(
        "horizontal * depth {}",
        position.horizontal * position.depth
    );
}

fn update_position(position: &mut Position, command: &str) {
    let mut parts = command.split_ascii_whitespace();
    let command = parts.next().unwrap();
    let quantity: i32 = parts.next().unwrap().parse().unwrap();

    match command {
        "up" => position.depth -= quantity,
        "down" => position.depth += quantity,
        _ => position.horizontal += quantity,
    };
}
