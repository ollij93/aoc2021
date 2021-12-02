extern crate argparse;

mod common;
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
    let mut day: u32 = 0;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run Advent of Code solutions.");
        ap.refer(&mut day).add_option(
            &["-d", "--day"],
            Store,
            "Advent day to run, if not specified all are run in sequence",
        );
        ap.parse_args_or_exit();
    }

    match day {
        1 => day1::run(input_as_u32()),
        2 => day2::run(input_as_string()),
        _ => {
            println!("Day not yet implemented.");
        }
    }
}
