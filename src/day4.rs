// Solutions for day3 of Advent of Code

use super::common::run_and_print_time;

mod board;
mod game;

use game::parse_game;
use game::BingoGame;

fn p1(input: &Vec<String>) -> u32 {
    let mut game: BingoGame = parse_game(input);
    game.run()[0]
}

fn p2(input: &Vec<String>) -> u32 {
    let mut game: BingoGame = parse_game(input);
    *game.run().last().unwrap()
}

pub fn run(input: Vec<String>) {
    let a = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, &input);
    println!("Part2: {}", b);
}
