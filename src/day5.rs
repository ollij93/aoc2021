// Solutions for day5 of Advent of Code

use super::common::run_and_print_time;
use std::collections::HashMap;

mod lines;

fn get_count(input: &Vec<String>, inc_diags: bool) -> u32 {
    //game::parse_game(input).run()[0]
    let lines = lines::parse_lines(input);
    let mut hashmap = HashMap::new();
    for line in lines {
        let points = line.points(inc_diags);
        for point in points {
            let value = 1 + hashmap.get(&point).unwrap_or(&0);
            hashmap.insert(point, value);
        }
    }
    let mut count = 0;
    for (_, val) in hashmap.iter() {
        if *val > 1 {
            count += 1;
        }
    }
    count
}

fn p1(input: &Vec<String>) -> u32 {
    get_count(input, false)
}

fn p2(input: &Vec<String>) -> u32 {
    get_count(input, true)
}

pub fn run(input: Vec<String>) {
    let a = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, &input);
    println!("Part2: {}", b);
}
