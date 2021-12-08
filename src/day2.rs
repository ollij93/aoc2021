// Solutions for day2 of Advent of Code

use super::common::run_and_print_time;

enum SubmarineCommand {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_pairs(input: Vec<(String, u32)>) -> Vec<SubmarineCommand> {
    input
        .iter()
        .map(|(cmd, n)| match cmd.as_ref() {
            "forward" => SubmarineCommand::Forward(*n),
            "down" => SubmarineCommand::Down(*n),
            "up" => SubmarineCommand::Up(*n),
            x => panic!("'{}' is not a valid command", x),
        })
        .collect()
}

fn parse_input(input: &[String]) -> Vec<(String, u32)> {
    input
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            (String::from(parts[0]), parts[1].parse::<u32>().unwrap())
        })
        .collect()
}

pub fn p1(input: &[String]) -> u32 {
    let pairs = parse_input(input);
    let commands = parse_pairs(pairs);

    let pos = commands.iter().fold((0, 0), |pos, cmd| match cmd {
        SubmarineCommand::Forward(x) => (pos.0 + x, pos.1),
        SubmarineCommand::Down(x) => (pos.0, pos.1 + x),
        SubmarineCommand::Up(x) => (pos.0, pos.1 - x),
    });

    pos.0 * pos.1
}

pub fn p2(input: &[String]) -> u32 {
    let pairs = parse_input(input);
    let commands = parse_pairs(pairs);

    let (_, pos) = commands
        .iter()
        .fold((0, (0, 0)), |(aim, pos), cmd| match cmd {
            SubmarineCommand::Forward(x) => (aim, (pos.0 + x, pos.1 + aim * x)),
            SubmarineCommand::Down(x) => (aim + x, pos),
            SubmarineCommand::Up(x) => (aim - x, pos),
        });

    pos.0 * pos.1
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 2 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}