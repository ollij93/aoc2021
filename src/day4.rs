// Solutions for day4 of Advent of Code

use super::common::run_and_print_time;

mod board;
mod game;

fn p1(input: &[String]) -> u32 {
    game::parse_game(input).run()[0]
}

fn p2(input: &[String]) -> u32 {
    *game::parse_game(input).run().last().unwrap()
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 4 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}