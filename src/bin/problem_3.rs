//! Advent of code problem 3
//!
//! Written by Trevor Leibert

use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/input_3.txt").unwrap();

    // let ans = calculate_power_usage(&contents);
    // println!("Power consumption: {}", ans);

    let ans = calculate_life_support_rating(&contents);
    println!("Life support rating: {}", ans);
}

// fn calculate_power_usage(data: &str) -> usize {
//     let line_len = data.lines().next().unwrap().len();

//     let mut counts = vec![0; line_len];

//     let mut line_count = 0;
//     // iterate over the rest of the lines
//     for line in data.lines() {
//         for (idx, c) in line.chars().enumerate() {
//             counts[idx] += (c == '1') as usize;
//         }
//         line_count += 1;
//     }

//     // bit of gamma is most common bit, bit of epsilon is least common bit
//     let mut gamma: usize = 0;
//     let mut epsilon: usize = 0;

//     for (idx, count) in counts.iter().enumerate() {
//         // one is most common if the count of ones is over num_lines / 2
//         let one_common = (count / (line_count / 2)) != 0;
//         let shift_val = line_len - idx - 1;
//         gamma |= (one_common as usize) << shift_val;
//         epsilon |= (!one_common as usize) << shift_val;
//     }

//     gamma * epsilon
// }

fn calculate_life_support_rating(data: &str) -> usize {
    let line_len = data.lines().next().unwrap().len();
    let mut oxygen_candidates: Vec<_> = data
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();
    let mut co2_candidates = oxygen_candidates.clone();

    let mut idx: usize = 0;
    while oxygen_candidates.len() > 1 && idx < line_len {
        let len = oxygen_candidates.len();
        let mask = 1 << (line_len - idx - 1);

        let one_count = oxygen_candidates
            .iter()
            .filter(|num| *num & mask != 0)
            .count();
        let zero_count = len - one_count;

        let most_common = if one_count >= zero_count { 1 } else { 0 };

        oxygen_candidates = oxygen_candidates
            .into_iter()
            .filter(|num| (num & mask != 0) == (most_common == 1))
            .collect();

        idx += 1;
    }

    idx = 0;
    while co2_candidates.len() > 1 && idx < line_len {
        let len = co2_candidates.len();
        let mask = 1 << (line_len - idx - 1);

        let one_count = co2_candidates.iter().filter(|num| *num & mask != 0).count();
        let zero_count = len - one_count;

        let least_common = if one_count < zero_count { 1 } else { 0 };

        co2_candidates = co2_candidates
            .into_iter()
            .filter(|num| (num & mask == 0) == (least_common == 0))
            .collect();

        idx += 1;
    }

    assert!(
        oxygen_candidates.len() == 1 && co2_candidates.len() == 1,
        "Didn't narrow input down to 2 numbers!"
    );
    oxygen_candidates[0] * co2_candidates[0]
}
