//! Advent of Code problem 13
//!
//! Written By Trevor Leibert

use std::{fmt::Debug, fs};

pub struct Paper {
    paper: Vec<Vec<bool>>,
}

impl Paper {
    pub fn new(positions: &str) -> Self {
        let max_x = positions
            .lines()
            .map(|line| line.split(',').next().unwrap().parse::<usize>().unwrap())
            .max()
            .unwrap();
        let max_y = positions
            .lines()
            .map(|line| {
                line.split(',')
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .max()
            .unwrap();

        let mut paper = vec![vec![false; max_x + 1]; max_y + 1];
        // populate the vec
        for line in positions.lines() {
            let mut split = line.split(',');
            let x: usize = split.next().unwrap().parse().unwrap();
            let y: usize = split.next().unwrap().parse().unwrap();
            paper[y][x] = true;
        }
        Self { paper }
    }

    pub fn fold(&mut self, instruction: &str) {
        let coord = instruction
            .split('=')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        if instruction.contains('x') {
            self.fold_left(coord);
        } else {
            self.fold_up(coord);
        }
    }

    fn fold_up(&mut self, y: usize) {
        let y_dist = self.y_len() - y;
        for y_idx in 1..y_dist {
            for x_idx in 0..self.x_len() {
                self.paper[y - y_idx][x_idx] |= self.paper[y + y_idx][x_idx];
            }
        }

        // shorten the array
        self.paper.truncate(y);
    }

    fn fold_left(&mut self, x: usize) {
        let x_dist = self.x_len() - x;
        for x_idx in 1..x_dist {
            for y_idx in 0..self.y_len() {
                self.paper[y_idx][x - x_idx] |= self.paper[y_idx][x + x_idx];
            }
        }

        // shorten the array
        for row in &mut self.paper {
            row.truncate(x);
        }
    }

    pub fn y_len(&self) -> usize {
        return self.paper.len();
    }

    pub fn x_len(&self) -> usize {
        return self.paper[0].len();
    }

    pub fn count_dots(&self) -> usize {
        self.paper
            .iter()
            .map(|row| row.iter().filter(|val| **val).count())
            .sum()
    }
}

impl Debug for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .paper
            .iter()
            .map(|line| {
                line.iter()
                    .map(|val| if *val { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_13.txt").unwrap();
    let mut split = contents.split("\n\n");
    let positions = split.next().unwrap();
    let instructions = split.next().unwrap();
    let mut paper = Paper::new(positions);

    println!("{:?}\n\n", &paper);

    for instruction in instructions.lines() {
        paper.fold(instruction);
        // break;
    }

    println!("{:?}", &paper);

    // println!("Number of dots after one fold: {}", paper.count_dots());
}
