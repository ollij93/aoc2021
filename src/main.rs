extern crate argparse;

mod day1;

use argparse::{ArgumentParser, Store};

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
        (1, 1) => { day1::p1(); }
        (1, 2) => { day1::p2(); }
        _ => { println!("Day/Part combo not yet implemented."); }
    }
}