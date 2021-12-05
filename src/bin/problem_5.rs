//! Advent of Code problem 5
//!
//! Written by Trevor Leibert

use std::cmp::Ordering;
use std::fs;

pub struct Field {
    grid: Vec<Vec<u8>>,
}

impl Field {
    pub fn new(dim: usize) -> Self {
        Self {
            grid: vec![vec![0; dim]; dim],
        }
    }

    fn parse_str(line: &str) -> ((usize, usize), (usize, usize)) {
        let mut split = line.split("->");
        let part_1 = split.next().unwrap();
        let part_2 = split.next().unwrap();

        let mut split = part_1.trim().split(',');
        let start_x: usize = split.next().unwrap().parse().unwrap();
        let start_y: usize = split.next().unwrap().parse().unwrap();

        let mut split = part_2.trim().split(',');
        let end_x: usize = split.next().unwrap().parse().unwrap();
        let end_y: usize = split.next().unwrap().parse().unwrap();
        ((start_x, start_y), (end_x, end_y))
    }

    pub fn add_line_no_diagonal(&mut self, line: &str) {
        let ((mut y_pos, mut x_pos), (end_x, end_y)) = Self::parse_str(line);

        if y_pos == end_y {
            // drawing horizontal line
            while x_pos != end_x {
                self.grid[y_pos][x_pos] += 1;
                match x_pos.cmp(&end_x) {
                    Ordering::Greater => x_pos -= 1,
                    Ordering::Less => x_pos += 1,
                    Ordering::Equal => (),
                }
            }

            self.grid[y_pos][x_pos] += 1;
        } else if x_pos == end_x {
            // drawing vertical line
            while y_pos != end_y {
                self.grid[y_pos][x_pos] += 1;
                match y_pos.cmp(&end_y) {
                    Ordering::Greater => y_pos -= 1,
                    Ordering::Less => y_pos += 1,
                    Ordering::Equal => (),
                }
            }

            self.grid[y_pos][x_pos] += 1;
        }
    }

    pub fn add_line_diagonal(&mut self, line: &str) {
        // drawing the dreaded diagonal line
        let ((mut x_pos, mut y_pos), (end_x, end_y)) = Self::parse_str(line);

        while x_pos != end_x || y_pos != end_y {
            self.grid[y_pos][x_pos] += 1;
            match x_pos.cmp(&end_x) {
                Ordering::Greater => x_pos -= 1,
                Ordering::Less => x_pos += 1,
                Ordering::Equal => (),
            }

            match y_pos.cmp(&end_y) {
                Ordering::Greater => y_pos -= 1,
                Ordering::Less => y_pos += 1,
                Ordering::Equal => (),
            }
        }

        // need to draw the last one, since the line is inclusive
        self.grid[y_pos][x_pos] += 1;
    }

    pub fn num_intersects(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|element| **element > 1).count())
            .sum()
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_5.txt").unwrap();

    let mut field = Field::new(1000);
    for line in contents.lines() {
        field.add_line_diagonal(line);
    }

    let ans = field.num_intersects();
    println!("Total count: {}", ans);
}
