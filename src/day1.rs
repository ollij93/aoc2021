// Solutions for day1 of Advent of Code

use super::common::run_and_print_time;

fn p1(input: &Vec<u32>) -> u32 {
    let mut ret: u32 = 0;

    for (a, b) in input.iter().zip(&input[1..]) {
        if a < b {
            ret += 1;
        }
    }

    return ret;
}

fn p2(input: &Vec<u32>) -> u32 {
    // Calculate the 3 element sums and then re-run part1 on those values.
    let mut sums: Vec<u32> = Vec::new();
    for ((a, b), c) in input.iter().zip(&input[1..]).zip(&input[2..]) {
        sums.push(a + b + c);
    }

    return p1(&sums);
}

pub fn run(input: Vec<u32>) {
    let a = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, &input);
    println!("Part2: {}", b);
}
