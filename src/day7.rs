// Solutions for day7 of Advent of Code

use super::common::{mean, median, run_and_print_time};

fn calculate_cost<F>(input: &[u32], tgt: u32, cost_to_move: &F) -> u32
where
    F: Fn(u32) -> u32,
{
    input.iter().fold(0, |cost, pos| {
        cost + cost_to_move(if *pos < tgt { tgt - pos } else { pos - tgt })
    })
}

fn p1(input: Vec<u32>) -> u32 {
    calculate_cost(&input, median(&input), &|x| x)
}

fn p2_cost_to_move(units: u32) -> u32 {
    if units == 0 {
        0
    } else {
        units + p2_cost_to_move(units - 1)
    }
}

fn p2(input: Vec<u32>) -> u32 {
    calculate_cost(&input, mean(&input), &p2_cost_to_move)
}

pub fn run(input: Vec<u32>) {
    let a = run_and_print_time(p1, input.clone());
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, input);
    println!("Part2: {}", b);
}
