//! Advent of Code problem 13
//!
//! Written by Trevor Leibert

use std::{
    collections::{hash_map::Entry, HashMap},
    fs, str,
};

#[derive(Debug)]
pub struct Polymer {
    chars: HashMap<char, usize>,
    pairs: HashMap<String, usize>,
    rules: HashMap<String, char>,
}

struct Diff {
    new_pairs: [String; 2],
    remove_pair: String,
    letter: char,
    quantity: usize,
}

impl Polymer {
    pub fn new(state: &str, rules: &str) -> Self {
        let mut chars = HashMap::new();
        for c in state.chars() {
            chars.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut pairs = HashMap::new();

        // get string slice windows assuming ascii string
        for pair in state
            .as_bytes()
            .windows(2)
            .map(|win| str::from_utf8(win).unwrap())
        {
            let pair = pair.to_string();
            match pairs.entry(pair) {
                Entry::Occupied(o) => *o.into_mut() += 1,
                Entry::Vacant(v) => {
                    v.insert(1);
                }
            };
        }

        let mut rule_map = HashMap::new();
        for rule in rules.lines() {
            let (pair, res) = rule.split_once("->").unwrap();
            let pair = pair.trim().to_string();
            let res = res.trim().chars().next().unwrap();

            rule_map.insert(pair, res);
        }

        Self {
            chars,
            pairs,
            rules: rule_map,
        }
    }

    pub fn expand_once(&mut self) {
        let mut diffs = Vec::new();
        for (pair, new) in &self.rules {
            if let Some(&count) = self.pairs.get(pair) {
                if count == 0 {
                    continue;
                }

                let mut first = (&pair[..1]).to_string();
                first.push(*new);
                let mut second = new.to_string();
                second.push_str(&pair[1..]);

                let to_remove = pair.clone();

                let diff = Diff {
                    new_pairs: [first, second],
                    remove_pair: to_remove,
                    letter: *new,
                    quantity: count,
                };

                diffs.push(diff);
            }
        }

        for diff in diffs {
            self.update(diff);
        }
    }

    fn update(&mut self, diff: Diff) {
        self.chars
            .entry(diff.letter)
            .and_modify(|count| *count += diff.quantity)
            .or_insert(diff.quantity);

        self.pairs
            .entry(diff.remove_pair)
            .and_modify(|count| *count -= diff.quantity);

        for pair in diff.new_pairs {
            self.pairs
                .entry(pair)
                .and_modify(|count| *count += diff.quantity)
                .or_insert(diff.quantity);
        }
    }

    pub fn len(&self) -> usize {
        return self.chars.values().sum();
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_14.txt").unwrap();
    let mut split = contents.split("\n\n");
    let initializer = split.next().unwrap();
    let instructions = split.next().unwrap();

    let mut polymer = Polymer::new(initializer, instructions);
    println!("Polymer length at start: {}", polymer.len());

    for _ in 0..10 {
        polymer.expand_once();
    }
    println!("Polymer length after 10 iterations: {}", polymer.len());

    let max_min_diff =
        *polymer.chars.values().max().unwrap() - *polymer.chars.values().min().unwrap();
    println!(
        "Diff between min and max after 10 iterations: {}",
        max_min_diff
    );

    for _ in 0..30 {
        polymer.expand_once();
    }
    println!("Polymer length after 40 iterations: {}", polymer.len());

    let max_min_diff =
        *polymer.chars.values().max().unwrap() - *polymer.chars.values().min().unwrap();
    println!(
        "Diff between min and max after 40 iterations: {}",
        max_min_diff
    );
}
