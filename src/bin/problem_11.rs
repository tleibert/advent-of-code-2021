//! Advent of Code problem 11
//!
//! Written by Trevor Leibert
//! Yes I know Octopode is the wrong word, but it's more fun to write.

use std::{collections::VecDeque, fs};

#[derive(Debug)]
pub struct Octopode {
    field: Vec<Vec<u8>>,
}

impl Octopode {
    pub fn new(initial_state: &str) -> Self {
        let field = initial_state
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        Self { field }
    }

    /// progresses the state of the simulation by one time step,
    /// returning the number of octopodes that have flashed
    pub fn step(&mut self) -> usize {
        let mut to_flash = VecDeque::new();
        for (y, row) in self.field.iter_mut().enumerate() {
            for (x, num) in row.iter_mut().enumerate() {
                *num += 1;
                if *num == 10 {
                    to_flash.push_back((x, y));
                }
            }
        }

        let mut flashes = 0;

        while to_flash.len() > 0 {
            let (x, y) = to_flash.pop_front().unwrap();
            self.flash(x, y, &mut to_flash);
            flashes += 1;
        }

        for row in &mut self.field {
            for num in row {
                if *num > 9 {
                    *num = 0;
                }
            }
        }

        flashes
    }

    /// increment surrounding octopodes
    fn flash(&mut self, x: usize, y: usize, to_flash: &mut VecDeque<(usize, usize)>) {
        // increment above if in bounds, with diagonals if they're in bounds
        if y > 0 {
            self.inc(x, y - 1, to_flash);

            if x > 0 {
                self.inc(x - 1, y - 1, to_flash);
            }

            if x < self.field[0].len() - 1 {
                self.inc(x + 1, y - 1, to_flash);
            }
        }

        // increment below if in bounds, with diagonals if they're in bounds
        if y < self.field.len() - 1 {
            self.inc(x, y + 1, to_flash);

            if x > 0 {
                self.inc(x - 1, y + 1, to_flash);
            }

            if x < self.field[0].len() - 1 {
                self.inc(x + 1, y + 1, to_flash);
            }
        }

        // increment left if in bounds
        if x > 0 {
            self.inc(x - 1, y, to_flash);
        }

        // increment right if in bounds
        if x < self.field[0].len() - 1 {
            self.inc(x + 1, y, to_flash);
        }
    }

    /// increments a field, and puts it in the deque if it got incremented to 10
    fn inc(&mut self, x: usize, y: usize, to_flash: &mut VecDeque<(usize, usize)>) {
        self.field[y][x] += 1;
        if self.field[y][x] == 10 {
            to_flash.push_back((x, y));
        }
    }

    /// Returns the number of octopodes in the group
    pub fn len(&self) -> usize {
        self.field[0].len() * self.field.len()
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_11.txt").unwrap();
    let mut octopodes = Octopode::new(&contents);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += octopodes.step();
    }
    println!("Number of flashes: {}", flashes);

    let mut octopodes = Octopode::new(&contents);
    let mut step = 0;
    let target_flashes = octopodes.len();

    loop {
        step += 1;
        if octopodes.step() == target_flashes {
            break;
        }
    }
    println!("First synchronized flash is on step {}", step);
}
