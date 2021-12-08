// Solutions for day5 of Advent of Code

use super::common::run_and_print_time;
use std::collections::HashMap;

mod lines;

fn get_count(input: &[lines::Line], inc_diags: bool) -> u32 {
    input
        .iter()
        // Get the full list of points in all lines
        .flat_map(|line| line.points(inc_diags))
        // Fold those into a hashmap of point -> count
        .fold(HashMap::new(), |mut hm, point| {
            *hm.entry(point).or_insert(0) += 1;
            hm
        })
        // Count the number of points with multiple entries
        .values()
        .filter(|n| **n > 1)
        .count() as u32
}

fn p1(lns: &[lines::Line]) -> u32 {
    get_count(lns, false)
}

fn p2(lns: &[lines::Line]) -> u32 {
    get_count(lns, true)
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 5 ===");

    let lines = lines::parse_lines(&input);

    let (a, timea) = run_and_print_time(p1, &lines);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &lines);
    println!("Part2: {}", b);

    timea + timeb
}
