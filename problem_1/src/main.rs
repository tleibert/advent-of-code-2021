//! Advent of code problem 1
//!
//! Written by Trevor Leibert

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let fh = File::open("inputs/input_1.txt").unwrap();
    let reader = BufReader::new(fh);

    let ans = count_depth_increases_window(reader);

    println!("Number of depth increases: {}", ans);
}

fn count_depth_increases(reader: BufReader<File>) -> i32 {
    let mut count = -1; // don't count first line
    let mut prev = 0;

    for depth in reader.lines().map(|line| line.unwrap().parse().unwrap()) {
        if depth > prev {
            count += 1;
        }

        prev = depth;
    }

    count
}

fn count_depth_increases_window(reader: BufReader<File>) -> i32 {
    let mut count = -3; // first 3 values must be discarded
    let mut window = [0; 3];
    let mut prev_sum = 0;

    for depth in reader.lines().map(|line| line.unwrap().parse().unwrap()) {
        window[2] = window[1];
        window[1] = window[0];
        window[0] = depth;

        let cur_sum = window.iter().sum();
        if cur_sum > prev_sum {
            count += 1;
        }

        prev_sum = cur_sum;
    }

    count
}
