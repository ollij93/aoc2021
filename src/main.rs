extern crate argparse;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use argparse::{ArgumentParser, Store};
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn input_as_string() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

// Get input lines from stdin as a request type vector
fn input_as<I>() -> Vec<I>
where
    I: FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_as_string()
        .iter()
        .map(|line| line.parse::<I>().unwrap())
        .collect()
}

fn input_from_list_as<I>() -> Vec<I>
where
    I: FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_as_string()[0]
        .split(',')
        .map(|s| s.parse::<I>().unwrap())
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
        1 => day1::run(input_as::<u32>()),
        2 => day2::run(input_as_string()),
        3 => day3::run(input_as_string()),
        4 => day4::run(input_as_string()),
        5 => day5::run(input_as_string()),
        6 => day6::run(input_from_list_as::<u64>()),
        7 => day7::run(input_from_list_as::<u32>()),
        _ => {
            println!("Day not yet implemented.");
        }
    }
}
