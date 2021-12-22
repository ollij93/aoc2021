extern crate argparse;

mod common;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod point;

use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn input_as_string(reader: &mut dyn BufRead) -> Vec<String> {
    reader.lines().map(|line| line.unwrap()).collect()
}

// Get input lines from stdin as a request type vector
fn input_as<I>(reader: &mut dyn BufRead) -> Vec<I>
where
    I: FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_as_string(reader)
        .iter()
        .map(|line| line.parse::<I>().unwrap())
        .collect()
}

fn input_from_list_as<I>(reader: &mut dyn BufRead) -> Vec<I>
where
    I: FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_as_string(reader)[0]
        .split(',')
        .map(|s| s.parse::<I>().unwrap())
        .collect()
}

fn input_from_grid(reader: &mut dyn BufRead) -> Vec<Vec<u8>> {
    input_as_string(reader)
        .iter()
        .map(|line| line.chars().map(|c| (c as u8 - b'0')).collect::<Vec<u8>>())
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
        0 => {
            let total: u128 = [
                day1::run(input_as::<u32>(&mut BufReader::new(
                    File::open("inputs/day1.txt").unwrap(),
                ))),
                day2::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day2.txt").unwrap(),
                ))),
                day3::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day3.txt").unwrap(),
                ))),
                day4::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day4.txt").unwrap(),
                ))),
                day5::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day5.txt").unwrap(),
                ))),
                day6::run(input_from_list_as::<u64>(&mut BufReader::new(
                    File::open("inputs/day6.txt").unwrap(),
                ))),
                day7::run(input_from_list_as::<u32>(&mut BufReader::new(
                    File::open("inputs/day7.txt").unwrap(),
                ))),
                day8::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day8.txt").unwrap(),
                ))),
                day9::run(input_from_grid(&mut BufReader::new(
                    File::open("inputs/day9.txt").unwrap(),
                ))),
                day10::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day10.txt").unwrap(),
                ))),
                day11::run(input_from_grid(&mut BufReader::new(
                    File::open("inputs/day11.txt").unwrap(),
                ))),
                day12::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day12.txt").unwrap(),
                ))),
                day13::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day13.txt").unwrap(),
                ))),
                day14::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day14.txt").unwrap(),
                ))),
                day15::run(input_from_grid(&mut BufReader::new(
                    File::open("inputs/day15.txt").unwrap(),
                ))),
                day16::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day16.txt").unwrap(),
                ))),
                day17::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day17.txt").unwrap(),
                ))),
                day18::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day18.txt").unwrap(),
                ))),
                day19::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day19.txt").unwrap(),
                ))),
                day20::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day20.txt").unwrap(),
                ))),
                day21::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day21.txt").unwrap(),
                ))),
                day22::run(input_as_string(&mut BufReader::new(
                    File::open("inputs/day22.txt").unwrap(),
                ))),
            ]
            .iter()
            .sum::<u128>();
            println!("TOTAL TIME: {}µs", total);
        }
        1 => {
            day1::run(input_as::<u32>(&mut BufReader::new(io::stdin())));
        }
        2 => {
            day2::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        3 => {
            day3::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        4 => {
            day4::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        5 => {
            day5::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        6 => {
            day6::run(input_from_list_as::<u64>(&mut BufReader::new(io::stdin())));
        }
        7 => {
            day7::run(input_from_list_as::<u32>(&mut BufReader::new(io::stdin())));
        }
        8 => {
            day8::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        9 => {
            day9::run(input_from_grid(&mut BufReader::new(io::stdin())));
        }
        10 => {
            day10::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        11 => {
            day11::run(input_from_grid(&mut BufReader::new(io::stdin())));
        }
        12 => {
            day12::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        13 => {
            day13::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        14 => {
            day14::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        15 => {
            day15::run(input_from_grid(&mut BufReader::new(io::stdin())));
        }
        16 => {
            day16::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        17 => {
            day17::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        18 => {
            day18::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        19 => {
            day19::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        20 => {
            day20::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        21 => {
            day21::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        22 => {
            day22::run(input_as_string(&mut BufReader::new(io::stdin())));
        }
        _ => {
            println!("Day not yet implemented.");
        }
    }
}
