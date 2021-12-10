//! Advent of Code problem 10
//!
//! Written by Trevor Leibert

use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("inputs/input_10.txt").unwrap();
    let err_point = init_err_point_map();
    let auto_point = init_auto_point_map();

    let error_score = calculate_error_score(&contents, &err_point);
    println!("Total error score: {}", error_score);

    let mut auto_score = auto_complete_score(&contents, &auto_point);
    auto_score.sort();

    println!("Middle auto score: {}", auto_score[auto_score.len() / 2]);
}

fn init_err_point_map() -> HashMap<char, usize> {
    let mut err_point = HashMap::new();
    err_point.insert(')', 3);
    err_point.insert(']', 57);
    err_point.insert('}', 1197);
    err_point.insert('>', 25137);
    err_point
}

fn init_auto_point_map() -> HashMap<char, usize> {
    let mut err_point = HashMap::new();
    err_point.insert(')', 1);
    err_point.insert(']', 2);
    err_point.insert('}', 3);
    err_point.insert('>', 4);
    err_point
}

fn calculate_error_score(data: &str, err_point: &HashMap<char, usize>) -> usize {
    data.lines()
        .map(|line| {
            let mut stack = Vec::new();
            for char in line.chars() {
                match char {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    c if err_point.contains_key(&c) => {
                        if stack.pop() != Some(c) {
                            return *err_point.get(&c).unwrap();
                        }
                    }
                    _ => panic!("Got bad character {}", char),
                }
            }
            0
        })
        .sum()
}

fn auto_complete_score(data: &str, auto_point: &HashMap<char, usize>) -> Vec<usize> {
    data.lines()
        .map(|line| {
            let mut stack = Vec::new();
            for char in line.chars() {
                match char {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    c if auto_point.contains_key(&c) => {
                        if stack.pop() != Some(c) {
                            return None;
                        }
                    }
                    _ => panic!("Got bad character {}", char),
                }
            }

            Some(
                stack
                    .into_iter()
                    .rev()
                    .fold(0, |total, char| total * 5 + auto_point.get(&char).unwrap()),
            )
        })
        .filter_map(|score| score)
        .collect()
}
