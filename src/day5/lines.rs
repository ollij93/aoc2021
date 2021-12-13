use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;

use crate::point::Point;

#[derive(PartialEq, Eq, Hash)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?} -> {:?}", self.start, self.end)
    }
}

fn get_step(start: u32, end: u32) -> i32 {
    match start.cmp(&end) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

impl Line {
    pub fn points(&self, inc_diags: bool) -> Vec<Point> {
        if !(inc_diags || self.start.x == self.end.x || self.start.y == self.end.y) {
            return vec![];
        }
        let stepx = get_step(self.start.x, self.end.x);
        let stepy = get_step(self.start.y, self.end.y);

        let (mut curx, mut cury) = (self.start.x as i32, self.start.y as i32);
        let mut points = vec![];
        while (curx, cury) != (self.end.x as i32 + stepx, self.end.y as i32 + stepy) {
            points.push(Point {
                x: curx as u32,
                y: cury as u32,
            });
            curx += stepx;
            cury += stepy;
        }
        points
    }
}

fn parse_point(pointstr: &str) -> Point {
    let nums: Vec<u32> = pointstr
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    Point {
        x: nums[0],
        y: nums[1],
    }
}

fn parse_line(line: &str) -> Line {
    let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    Line {
        start: parse_point(parts[0]),
        end: parse_point(parts[1]),
    }
}

pub fn parse_lines(input: &[String]) -> Vec<Line> {
    input.iter().map(|s| parse_line(s)).collect()
}
