//! Advent of Code problem 9
//!
//! Written by Trevor Leibert

use std::fs;

/// Struct implementing the height map and useful functions on it
#[derive(Debug)]
struct HeightMap {
    data: Vec<Vec<u8>>,
    xlen: usize,
    ylen: usize,
}

impl HeightMap {
    /// creates a heightmap from a rectangular grid of the chars 0-9
    pub fn new(data: &str) -> Self {
        let data: Vec<Vec<u8>> = data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|num| num.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let xlen = data[0].len();
        let ylen = data.len();

        Self { data, xlen, ylen }
    }

    /// returns the indicies of all the local minima in the heightmap
    pub fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();

        for (y, row) in self.data.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                let num = *num;
                let mut above_greater = true;
                let mut left_greater = true;
                let mut right_greater = true;
                let mut below_greater = true;

                if y > 0 {
                    above_greater = num < self.get(x, y - 1);
                }

                if y < self.ylen - 1 {
                    below_greater = num < self.get(x, y + 1);
                }

                if x > 0 {
                    left_greater = num < self.get(x - 1, y);
                }

                if x < self.xlen - 1 {
                    right_greater = num < self.get(x + 1, y);
                }

                if above_greater && left_greater && below_greater && right_greater {
                    ret.push((x, y));
                }
            }
        }

        ret
    }

    /// gets a copy of the data in the point
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    /// finds the area of the basin containing this value
    pub fn basin_area(&self, x: usize, y: usize) -> usize {
        // let's keep a record of searched areas
        let mut searched = vec![vec![false; self.xlen]; self.ylen];
        self.search(&mut searched, x, y);

        // the number of areas we've searched is the area of the basin
        searched
            .into_iter()
            .map(|row| row.into_iter().filter(|val| *val).count())
            .sum()
    }

    /// helper recursive function to search through a basin
    fn search(&self, searched: &mut Vec<Vec<bool>>, x: usize, y: usize) {
        searched[y][x] = true;
        // we want to search in a direction if:
        // - we aren't at that edge
        // - we haven't looked there
        // - if the value there isn't 9

        // search above
        if y > 0 && !searched[y - 1][x] && self.get(x, y - 1) < 9 {
            self.search(searched, x, y - 1);
        }
        // search below
        if y < self.ylen - 1 && !searched[y + 1][x] && self.get(x, y + 1) < 9 {
            self.search(searched, x, y + 1);
        }
        // search to the left
        if x > 0 && !searched[y][x - 1] && self.get(x - 1, y) < 9 {
            self.search(searched, x - 1, y);
        }
        // search to the right
        if x < self.xlen - 1 && !searched[y][x + 1] && self.get(x + 1, y) < 9 {
            self.search(searched, x + 1, y);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_9.txt").unwrap();
    let heightmap = HeightMap::new(&contents);

    let low_points = heightmap.find_low_points();
    let risk_sum: usize = low_points
        .iter()
        .map(|(x, y)| (heightmap.get(*x, *y) + 1) as usize)
        .sum();
    println!("Sum of all risk levels: {}", risk_sum);

    let mut basin_areas: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| heightmap.basin_area(*x, *y))
        .collect();
    basin_areas.sort();
    let three_biggest_sum: usize = basin_areas.iter().rev().take(3).product();
    println!("Product of biggest 3 basins: {}", three_biggest_sum);
}
