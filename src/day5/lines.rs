use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;

pub struct Point {
    pub x: u32,
    pub y: u32,
}
impl Hash for Point {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        h.write_u32(self.x);
        h.write_u32(self.y);
    }
}
impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}
impl Eq for Point {}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl PartialEq for Line {
    fn eq(&self, rhs: &Line) -> bool {
        self.start == rhs.start && self.end == rhs.end
    }
}
impl Eq for Line {}
impl Hash for Line {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.start.hash(h);
        self.end.hash(h);
    }
}
impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?} -> {:?}", self.start, self.end)
    }
}

fn get_step(start: u32, end: u32) -> i32 {
    if start < end {
        1
    } else if start > end {
        -1
    } else {
        0
    }
}

impl Line {
    pub fn points(&self, inc_diags: bool) -> Vec<Point> {
        if !inc_diags && !(self.start.x == self.end.x || self.start.y == self.end.y) {
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
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    Point {
        x: nums[0],
        y: nums[1],
    }
}

fn parse_line(line: &String) -> Line {
    let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    Line {
        start: parse_point(parts[0]),
        end: parse_point(parts[1]),
    }
}

pub fn parse_lines(input: &Vec<String>) -> Vec<Line> {
    input.iter().map(parse_line).collect()
}
