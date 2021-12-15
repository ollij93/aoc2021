// Solutions for day14 of Advent of Code

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

use super::common::run_and_print_time;

type Pair = (char, char);

fn parse_rule(s: &str) -> (Pair, char) {
    let (keys, val) = s.split_once(" -> ").unwrap();
    let chars = keys.as_bytes();
    let keya = chars[0] as char;
    let keyb = chars[1] as char;
    ((keya, keyb), val.as_bytes()[0] as char)
}

fn parse_rules(input: &[String]) -> HashMap<Pair, char> {
    input.iter().map(|s| parse_rule(s)).collect()
}

fn parse_input(input: &[String]) -> (HashMap<Pair, u128>, HashMap<Pair, char>) {
    let polymer: Vec<char> = input[0].chars().collect();
    let mut polymermap: HashMap<Pair, u128> = HashMap::new();
    for pair in polymer.iter().zip(&polymer[1..]) {
        *polymermap.entry((*pair.0, *pair.1)).or_insert(0) += 1;
    }
    let rules: HashMap<Pair, char> = parse_rules(&input[2..]);
    (polymermap, rules)
}

fn get_strength(input: &[String], iterations: u128) -> u128 {
    let last_char = input[0].chars().last().unwrap();
    let (init_polymer, rules) = parse_input(input);
    let final_polymer = (0..iterations).fold(init_polymer, |polymer, _| {
        let new_pairs: Vec<(Pair, u128)> = polymer
            .iter()
            .map(|((a, b), val)| {
                let c = rules.get(&(*a, *b));
                match c {
                    None => vec![((*a, *b), *val)],
                    Some(x) => vec![((*a, *x), *val), ((*x, *b), *val)],
                }
            })
            .flatten()
            .collect();
        let mut new_polymer: HashMap<Pair, u128> = HashMap::new();
        for (pair, val) in new_pairs {
            *new_polymer.entry(pair).or_insert(0) += val;
        }
        new_polymer
    });

    // We know the first and last character haven't changed
    // Count the first character in all pairs will get us the true counts.
    // Just need to add one to whatever the last character is
    let mut counts: HashMap<char, u128> = HashMap::new();
    for ((x, _), val) in &final_polymer {
        *counts.entry(*x).or_insert(0) += val;
    }
    *counts.entry(last_char).or_insert(0) += 1;

    let max_val = counts.values().fold(0, |curr, this| max(*this, curr));
    let min_val = counts
        .values()
        .fold(std::u128::MAX, |curr, this| min(*this, curr));
    max_val - min_val
}

fn p1(input: &[String]) -> u128 {
    get_strength(input, 10)
}

fn p2(input: &[String]) -> u128 {
    get_strength(input, 40)
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 14 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
