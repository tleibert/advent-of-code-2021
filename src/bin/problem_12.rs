//! Advent of Code problem 12
//!
//! Written by Trevor Leibert
//! This solution can be sped up pretty drastically by pre-allocating
//! enough storage for all the solution paths, but that feels
//! a bit like cheating.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;

pub struct Graph<'a> {
    node_idx_map: HashMap<&'a str, usize>,
    idx_node_map: HashMap<usize, &'a str>,
    adj_matrix: Vec<Vec<u8>>,
    len: usize,
}

type CaveNetwork<'a> = Graph<'a>;

impl<'a> Debug for Graph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top: String = "         ".to_string()
            + &(0..self.len)
                .into_iter()
                .map(|i| format!(" {:5}", self.idx_node_map[&i]))
                .collect::<String>();

        let rows: String = (0..self.len)
            .into_iter()
            .map(|i| {
                format!("{:5}", self.idx_node_map[&i])
                    + &self.adj_matrix[i]
                        .iter()
                        .map(|num| format!(" {:5}", num))
                        .collect::<String>()
                    + "\n"
            })
            .collect();
        writeln!(f, "{}", top)?;
        write!(f, "{}", rows)
    }
}

impl<'a> Graph<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut nodes = HashSet::new();
        for line in data.lines() {
            for part in line.split('-') {
                nodes.insert(part);
            }
        }
        let len = nodes.len();
        let mut counter = 0;
        let node_idx_map: HashMap<&str, usize> = nodes
            .into_iter()
            .map(|name| {
                let entry = (name, counter);
                counter += 1;
                entry
            })
            .collect();

        let mut adj_matrix = vec![vec![0; len]; len];
        for line in data.lines() {
            let mut split = line.split('-');
            let start = split.next().unwrap();
            let end = split.next().unwrap();

            let start = node_idx_map[start];
            let end = node_idx_map[end];

            adj_matrix[start][end] = 1;
            // undirected graph so set other side the same
            adj_matrix[end][start] = 1;
        }

        let idx_node_map = node_idx_map.iter().map(|(k, v)| (*v, *k)).collect();

        Graph {
            node_idx_map,
            idx_node_map,
            adj_matrix,
            len,
        }
    }

    pub fn get_neighbors(&self, node: &str) -> HashSet<&str> {
        let node = self.node_idx_map[node];
        self.adj_matrix[node]
            .iter()
            .enumerate()
            .filter_map(|(idx, val)| {
                if *val == 1 {
                    Some(self.idx_node_map[&idx])
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_12.txt").unwrap();
    let caves = CaveNetwork::new(&contents);

    println!("{:?}", &caves);

    let paths = find_all_paths_1(&caves);
    println!("Number of paths part 1: {}", paths.len());

    let paths = find_all_paths_2(&caves);
    println!("Number of paths part 2: {}", paths.len());
}

/// finds every path through the cave starting at "start" and ending at "end"
/// only visiting small caves at most once
fn find_all_paths_1<'a>(caves: &'a CaveNetwork) -> HashSet<Vec<&'a str>> {
    let mut record = HashSet::new();

    visit_1(caves, "start", Vec::new(), &mut record);

    record
}

fn visit_1<'a>(
    caves: &'a CaveNetwork,
    node: &'a str,
    mut history: Vec<&'a str>,
    record: &mut HashSet<Vec<&'a str>>,
) {
    history.push(node);
    if node == "end" {
        record.insert(history);
        return;
    }

    let options = caves.get_neighbors(node);
    for option in options {
        // if we're looking at a lowercase/small cave we've seen before, don't go back
        if option.to_lowercase() == option && history.contains(&option) {
            continue;
        }

        // otherwise, visit this cave
        visit_1(caves, option, history.clone(), record);
    }
}

/// finds every path through the cave starting at "start" and ending at "end"
/// visiting one small cave at most twice, and all other small caves only once
fn find_all_paths_2<'a>(caves: &'a CaveNetwork) -> HashSet<Vec<&'a str>> {
    let mut record = HashSet::new();

    visit_2(caves, "start", Vec::new(), &mut record);

    record
}

fn visit_2<'a>(
    caves: &'a CaveNetwork,
    node: &'a str,
    mut history: Vec<&'a str>,
    record: &mut HashSet<Vec<&'a str>>,
) {
    history.push(node);
    if node == "end" {
        record.insert(history);
        return;
    }

    // check if we've visited small caves more than once
    let mut smalls = HashSet::new();
    let mut has_time = true;
    for item in &history {
        if item.to_lowercase() == *item {
            // if we've been in the same small cave twice, we don't
            // have time to do that again
            if !smalls.insert(*item) {
                has_time = false;
            }
        }
    }

    let options = caves.get_neighbors(node);
    for option in options {
        // if we're looking at the start, don't go there
        if option == "start" {
            continue;
        }

        // if we have not visited a small cave twice, we can go back to a small cave
        // otherwise, if we have visited a small cave twice, we cannot backtrack
        if option.to_lowercase() == option && !has_time && smalls.contains(option) {
            continue;
        }

        // otherwise, visit this cave
        visit_2(caves, option, history.clone(), record);
    }
}
