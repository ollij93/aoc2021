// Bingo game implementation for day4 of Advent of Code 2021

use super::board;
use super::board::BingoBoard;

pub struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl std::fmt::Display for BingoGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self.numbers).unwrap();
        for board in self.boards.iter() {
            writeln!(f, "{}", board).unwrap();
        }
        Ok(())
    }
}

impl BingoGame {
    pub fn run(mut self) -> Vec<u32> {
        let mut ret = Vec::new();
        for num in &self.numbers {
            self.boards = self
                .boards
                .iter()
                .map(|board| board.eliminate_num(*num))
                .collect();
            let mut indexes_to_remove: Vec<usize> = Vec::new();
            for (idx, board) in self.boards.iter().enumerate() {
                if board.is_solved() {
                    ret.push(board.sum() * num);
                    indexes_to_remove.push(idx);
                }
            }
            indexes_to_remove.sort_by(|a, b| b.cmp(a));
            for idx in indexes_to_remove {
                self.boards.remove(idx);
            }
        }
        ret
    }
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn parse_boards(input: &[String]) -> Vec<BingoBoard> {
    input
        .chunks(6)
        .map(|chnk| board::parse_board(chnk))
        .collect()
}

pub fn parse_game(input: &[String]) -> BingoGame {
    BingoGame {
        numbers: parse_numbers(&input[0]),
        boards: parse_boards(&input[1..]),
    }
}
