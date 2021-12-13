// Solutions for day13 of Advent of Code

use std::collections::HashSet;
use std::str::FromStr;

use super::common::run_and_print_time;
use super::point::Point;

enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match s.split_once("=") {
            None => Err(()),
            Some((instruction, position)) => {
                let pos = position.parse::<u32>();
                match (instruction, pos) {
                    ("fold along x", Ok(x)) => Ok(Fold::X(x)),
                    ("fold along y", Ok(y)) => Ok(Fold::Y(y)),
                    _ => Err(()),
                }
            }
        }
    }
}

fn parse_input(input: &[String]) -> (Vec<Point>, Vec<Fold>) {
    let points = input
        .iter()
        .map(|s| Point::from_str(s))
        .filter_map(|p| match p {
            Ok(point) => Some(point),
            _ => None,
        })
        .collect::<Vec<Point>>();

    let folds = input
        .iter()
        .map(|s| Fold::from_str(s))
        .filter_map(|f| match f {
            Ok(fold) => Some(fold),
            _ => None,
        })
        .collect::<Vec<Fold>>();

    (points, folds)
}

fn p1(input: &[String]) -> u32 {
    let (points, folds) = parse_input(input);
    points
        .iter()
        .map(|p| match folds[0] {
            Fold::X(x) => {
                if p.x > x {
                    Point {
                        x: 2 * x - p.x,
                        y: p.y,
                    }
                } else {
                    p.clone()
                }
            }
            Fold::Y(y) => {
                if p.y > y {
                    Point {
                        x: p.x,
                        y: 2 * y - p.y,
                    }
                } else {
                    p.clone()
                }
            }
        })
        .collect::<HashSet<Point>>()
        .len() as u32
}

fn p2(input: &[String]) -> String {
    let (init_points, folds) = parse_input(input);
    let final_points = folds.iter().fold(init_points, |points, fold| {
        points
            .iter()
            .map(|p| match fold {
                Fold::X(x) => {
                    if p.x > *x {
                        Point {
                            x: 2 * x - p.x,
                            y: p.y,
                        }
                    } else {
                        p.clone()
                    }
                }
                Fold::Y(y) => {
                    if p.y > *y {
                        Point {
                            x: p.x,
                            y: 2 * y - p.y,
                        }
                    } else {
                        p.clone()
                    }
                }
            })
            .collect::<Vec<Point>>()
    });

    let max_x = final_points
        .iter()
        .fold(0, |x, p| if p.x > x { p.x } else { x });
    let max_y = final_points
        .iter()
        .fold(0, |y, p| if p.y > y { p.y } else { y });
    (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| {
                    if final_points.contains(&Point { x, y }) {
                        "#".to_string()
                    } else {
                        ".".to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 13 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2:\n{}", b);

    timea + timeb
}
