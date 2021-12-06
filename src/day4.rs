// Solutions for day4 of Advent of Code

use super::common::run_and_print_time;

mod board;
mod game;

fn p1(input: &Vec<String>) -> u32 {
    game::parse_game(input).run()[0]
}

fn p2(input: &Vec<String>) -> u32 {
    *game::parse_game(input).run().last().unwrap()
}

pub fn run(input: Vec<String>) {
    let a = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, &input);
    println!("Part2: {}", b);
}
