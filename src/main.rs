extern crate argparse;

mod day1;

use argparse::{ArgumentParser, Store};
use std::io;
use std::io::BufRead;

// Get input lines from stdin as a u32 vector
fn input_as_u32() -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let value: u32 = line.unwrap().parse::<u32>().unwrap();
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
        (1, 1) => { day1::p1(input_as_u32()); }
        (1, 2) => { day1::p2(input_as_u32()); }
        _ => { println!("Day/Part combo not yet implemented."); }
    }
}