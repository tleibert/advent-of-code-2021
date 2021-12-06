//! Advent of Code problem 6
//!
//! Written by Trevor Leibert

use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/input_6.txt").unwrap();
    let initial_state = contents
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let n = 80;
    let ans = fish_calculus(&initial_state, n);

    println!("Number of fish after {} days: {}", n, ans);
}

fn fish_calculus(initial_state: &Vec<usize>, num_days: usize) -> usize {
    let mut fishes: [usize; 9] = [0; 9];

    for num in initial_state {
        fishes[*num] += 1;
    }

    for _ in 0..num_days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.iter().sum()
}
