// Solutions for day1 of Advent of Code

use super::common::run_and_print_time;

fn p1(input: &[u32]) -> usize {
    input.iter().zip(&input[1..]).filter(|(a, b)| a < b).count()
}

fn p2(input: &[u32]) -> usize {
    // Calculate the 3 element sums and then re-run part1 on those values.
    let sums = input
        .iter()
        .zip(&input[1..])
        .zip(&input[2..])
        .map(|((a, b), c)| a + b + c)
        .collect::<Vec<u32>>();

    p1(&sums)
}

pub fn run(input: Vec<u32>) -> u128 {
    println!("=== DAY 1 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
