//! Advent of Code problem 7
//!
//! Written by Trevor Leibert

use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/input_7.txt").unwrap();
    let crabs: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let ans = calculate_optimum_position(&crabs);
    println!("Crab position: {}", ans);
}

// /// Part 1 solution: the median minimizes the distance to all the other elements
// fn calculate_with_median(array: &mut [i32]) -> i32 {
//     array.sort();
//     println!("Length: {}", array.len());

//     let median = if array.len() % 2 == 0 {
//         let idx = array.len() / 2;
//         (array[idx - 1] + array[idx]) / 2
//     } else {
//         array[array.len() / 2]
//     };

//     array.iter().map(|num| (num - median).abs()).sum()
// }

/// Part 2 solution
fn calculate_optimum_position(array: &[i32]) -> i32 {
    (*array.iter().min().unwrap()..=*array.iter().max().unwrap())
        .into_iter()
        .map(|start| {
            array
                .iter()
                .map(|end| (1..=((*end - start).abs())).sum::<i32>())
                .sum()
        })
        .min()
        .unwrap()
}
