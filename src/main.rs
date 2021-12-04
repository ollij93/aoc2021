extern crate argparse;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

use argparse::{ArgumentParser, Store};
use std::io;
use std::io::BufRead;

fn input_as_string() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

// Get input lines from stdin as a u32 vector
fn input_as_u32() -> Vec<u32> {
    input_as_string()
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
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
        3 => day3::run(input_as_string()),
        4 => day4::run(input_as_string()),
        _ => {
            println!("Day not yet implemented.");
        }
    }
}
