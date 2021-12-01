//! Advent of code problem 1
//!
//! Written by Trevor Leibert

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");

    let fh = File::open("inputs/input_1.txt").unwrap();
    let reader = BufReader::new(fh);

    let ans = count_depth_increases(reader);

    println!("Number of depth increases: {}", ans);
}

fn count_depth_increases(reader: BufReader<File>) -> i32 {
    let mut sum = -1; // don't count first line
    let mut prev = 0;

    for depth in reader.lines().map(|line| line.unwrap().parse().unwrap()) {
        if depth > prev {
            sum += 1;
        }

        prev = depth;
    }

    sum
}
