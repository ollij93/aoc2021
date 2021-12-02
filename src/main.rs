extern crate argparse;

mod day1;
mod day2;

use argparse::{ArgumentParser, Store};
use std::io;
use std::io::BufRead;

fn input_as_string() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        vec.push(line.unwrap());
    }

    return vec;
}

// Get input lines from stdin as a u32 vector
fn input_as_u32() -> Vec<u32> {
    let input = input_as_string();
    let mut vec: Vec<u32> = Vec::new();

    for line in input {
        let value: u32 = line.parse::<u32>().unwrap();
        vec.push(value);
    }

    return vec;
}

fn main() {
    let mut day: i32 = 1;
    let mut part: i32 = 1;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run Advent of Code solutions.");
        ap.refer(&mut day).add_option(&["-d", "--day"], Store, "Advent day");
        ap.refer(&mut part).add_option(&["-p", "--part"], Store, "Part");
        ap.parse_args_or_exit();
    }

    match (day, part) {
        (1, 1) => { println!("{}", day1::p1(input_as_u32())); }
        (1, 2) => { println!("{}", day1::p2(input_as_u32())); }
        (2, 1) => { println!("{}", day2::p1(input_as_string()))}
        (2, 2) => { println!("{}", day2::p2(input_as_string()))}
        _ => { println!("Day/Part combo not yet implemented."); }
    }
}
