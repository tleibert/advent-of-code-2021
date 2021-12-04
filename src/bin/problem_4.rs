//! Advent of Code problem 4
//!
//! Written by Trevor Leibert

use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
struct Board {
    id: usize,
    board: [[usize; 5]; 5],
    board_transpose: [[usize; 5]; 5],
    space_map: HashMap<usize, (usize, usize)>, // map of numbers to board spaces
    won: bool,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = [[0; 5]; 5];
        for (num, (i, j)) in &self.space_map {
            grid[*i][*j] = *num;
        }
        let mut s = String::new();

        for (i, line) in grid.iter().enumerate() {
            for (j, num) in line.iter().enumerate() {
                if self.board[i][j] == 0 {
                    s.push_str(&num.to_string());
                } else {
                    s.push('x');
                }

                s.push(' ');
            }

            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl Board {
    fn new(id: usize, layout: &str) -> Self {
        let board = [[0; 5]; 5];
        let board_transpose = [[0; 5]; 5];
        let mut space_map = HashMap::new();
        for (i, line) in layout.lines().enumerate() {
            for (j, c) in line.split_whitespace().enumerate() {
                let num = c.parse().unwrap();
                space_map.insert(num, (i, j));
            }
        }

        Self {
            id,
            board,
            board_transpose,
            space_map,
            won: false,
        }
    }

    /// tries a move, returning if the move made this a winning board
    fn play(&mut self, num: usize) -> bool {
        if self.won {
            return true;
        }
        match self.space_map.get(&num) {
            Some((i, j)) => {
                self.board[*i][*j] = 1;
                self.board_transpose[*j][*i] = 1;
                let row_win = self.board.iter().any(|row| row.iter().sum::<usize>() == 5);
                let col_win = self
                    .board_transpose
                    .iter()
                    .any(|col| col.iter().sum::<usize>() == 5);
                self.won = row_win || col_win;
                self.won
            }
            None => false,
        }
    }

    fn score(&self, winning_call: usize) -> usize {
        let sum_unmarked: usize = self
            .space_map
            .iter()
            .map(|(num, (i, j))| {
                if self.board[*i][*j] == 0 {
                    *num as usize
                } else {
                    0
                }
            })
            .sum();

        sum_unmarked * winning_call
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input_4.txt").unwrap();
    let mut groups = contents.split("\n\n");
    let moves: Vec<usize> = groups
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();

    for (idx, part) in groups.enumerate() {
        boards.push(Board::new(idx, part))
    }

    let (winner, score) = play_game_last_winner(boards, &moves).unwrap();

    println!("Board {} won last with a score of {}", winner + 1, score);
}

// /// plays the game until a board wins, returning which board won, and with what score
// /// If no board won, returns None
// fn play_game_first_winner(mut boards: Vec<Board>, moves: &[usize]) -> Option<(usize, usize)> {
//     for play in moves {
//         for (idx, board) in boards.iter_mut().enumerate() {
//             if board.play(*play) {
//                 return Some((idx, board.score(*play)));
//             }
//         }
//     }

//     None
// }

fn play_game_last_winner(mut boards: Vec<Board>, moves: &[usize]) -> Option<(usize, usize)> {
    for play in moves {
        // only one board left, play it till it wins
        if boards.len() == 1 {
            if !boards[0].play(*play) {
                continue;
            };
            println!("{}", boards[0]);
            return Some((boards[0].id, boards[0].score(*play)));
        } else {
            let mut new_boards = Vec::new();
            for mut board in boards {
                let won = board.play(*play);
                if !won {
                    new_boards.push(board);
                }
            }
            boards = new_boards;
        }
    }
    None
}
