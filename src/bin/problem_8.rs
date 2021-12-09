//! Advent of Code problem 8
//!
//! Written by Trevor Leibert

use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

fn main() {
    let contents = fs::read_to_string("inputs/input_8.txt").unwrap();

    let ans = easy_digits(&contents);
    println!("Number of easy digits: {}", ans);

    let ans = full_solution(&contents);
    println!("Sum of all displays: {}", ans);
}

fn easy_digits(data: &str) -> usize {
    data.lines()
        .map(|line| {
            line.split('|')
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .filter(|entry| {
                    entry.len() == 2 || entry.len() == 4 || entry.len() == 3 || entry.len() == 7
                })
                .count()
        })
        .sum()
}

/// populates the map of numbers to the signals in them
fn populate_map(digit_data: &str) -> HashMap<usize, BTreeSet<char>> {
    let mut digit_map = HashMap::<usize, BTreeSet<char>>::new();
    let mut unclassified = HashMap::<usize, BTreeSet<BTreeSet<char>>>::new();

    // populate map with "easy" digits first
    for digit in digit_data.split_whitespace() {
        match digit.len() {
            2 => {
                digit_map.insert(1, digit.chars().collect());
            }
            4 => {
                digit_map.insert(4, digit.chars().collect());
            }
            3 => {
                digit_map.insert(7, digit.chars().collect());
            }
            7 => {
                digit_map.insert(8, digit.chars().collect());
            }
            len => {
                if !unclassified.contains_key(&len) {
                    unclassified.insert(len, BTreeSet::new());
                }

                unclassified
                    .get_mut(&len)
                    .unwrap()
                    .insert(digit.chars().collect());
            }
        }
    }

    // 6 is missing ONE of 1's values
    let six = unclassified[&6]
        .iter()
        .filter(|six_candidate| !six_candidate.is_superset(&digit_map[&1]))
        .next()
        .unwrap()
        .clone();
    unclassified.get_mut(&6).unwrap().remove(&six);
    digit_map.insert(6, six);

    // bottom right segment is the intersection of 1 and 6
    let lower_right = digit_map[&1]
        .intersection(&digit_map[&6])
        .next()
        .unwrap()
        .clone();

    // 3 is the only 5-segment number that has 1 contained in it
    let three = unclassified[&5]
        .iter()
        .filter(|three_candidate| three_candidate.is_superset(&digit_map[&1]))
        .next()
        .unwrap()
        .clone();
    unclassified.get_mut(&5).unwrap().remove(&three);
    digit_map.insert(3, three);

    // 5 is the only remaining 5-segment number with lower-right in it
    let five = unclassified[&5]
        .iter()
        .filter(|five_candidate| five_candidate.contains(&lower_right))
        .next()
        .unwrap()
        .clone();
    unclassified.get_mut(&5).unwrap().remove(&five);
    digit_map.insert(5, five);

    // 2 is the only remaining unclassified 5-segment number
    let two = unclassified[&5]
        .iter()
        .next()
        .unwrap()
        .iter()
        .cloned()
        .collect();
    unclassified.get_mut(&5).unwrap().remove(&two);
    digit_map.insert(2, two);

    // 9 is the only remaining 6-segment number with 3 as a subset
    let nine = unclassified[&6]
        .iter()
        .filter(|nine_candidate| nine_candidate.is_superset(&digit_map[&3]))
        .next()
        .unwrap()
        .clone();
    unclassified.get_mut(&6).unwrap().remove(&nine);
    digit_map.insert(9, nine);

    // 0 is the only remaining unclassified number, 6-segment or otherwise
    let zero = unclassified[&6]
        .iter()
        .next()
        .unwrap()
        .iter()
        .cloned()
        .collect();
    unclassified.get_mut(&6).unwrap().remove(&zero);
    digit_map.insert(0, zero);

    digit_map
}

/// finds the true output of each display and sums it
fn full_solution(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let mut parts = line.split('|');
            let digit_data = parts.next().unwrap();

            // create map of digits to the segment representing them
            let digit_map = populate_map(digit_data);

            // flip map so we can map from chars to number
            let char_map = digit_map
                .into_iter()
                .map(|(num, chars)| (chars, num))
                .collect::<HashMap<BTreeSet<char>, usize>>();

            let disp_data = parts.next().unwrap();
            let mut place_val = 10000;
            disp_data
                .split_whitespace()
                .map(|digit| {
                    place_val /= 10;
                    let converted = digit.chars().collect();
                    char_map[&converted] * place_val
                })
                .sum::<usize>()
        })
        .sum()
}
