// Solutions for day9 of Advent of Code

use super::common::run_and_print_time;
use std::collections::HashSet;

struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn up(&self, row: usize, col: usize) -> u8 {
        if row == 0 {
            9
        } else {
            self.heights[row - 1][col]
        }
    }

    fn down(&self, row: usize, col: usize) -> u8 {
        if row == self.heights.len() - 1 {
            9
        } else {
            self.heights[row + 1][col]
        }
    }

    fn left(&self, row: usize, col: usize) -> u8 {
        if col == 0 {
            9
        } else {
            self.heights[row][col - 1]
        }
    }

    fn right(&self, row: usize, col: usize) -> u8 {
        if col == self.heights[row].len() - 1 {
            9
        } else {
            self.heights[row][col + 1]
        }
    }

    fn lowpoints(&self) -> Vec<(usize, usize, u8)> {
        let mut ret = vec![];
        let collen = self.heights.len();
        for rowi in 0..collen {
            let rowlen = self.heights[rowi].len();
            for coli in 0..rowlen {
                let height = self.heights[rowi][coli];
                if self.up(rowi, coli) <= height
                    || self.down(rowi, coli) <= height
                    || self.left(rowi, coli) <= height
                    || self.right(rowi, coli) <= height
                {
                    continue;
                }
                ret.push((rowi, coli, height));
            }
        }
        ret
    }

    fn basinneighbours(&self, row: usize, col: usize) -> HashSet<(usize, usize)> {
        let mut ret = HashSet::from([(row, col)]);
        if self.left(row, col) < 9 {
            ret.insert((row, col - 1));
        }
        if self.right(row, col) < 9 {
            ret.insert((row, col + 1));
        }
        if self.up(row, col) < 9 {
            ret.insert((row - 1, col));
        }
        if self.down(row, col) < 9 {
            ret.insert((row + 1, col));
        }
        ret
    }

    fn basin(&self, row: usize, col: usize) -> HashSet<(usize, usize)> {
        let mut cur = HashSet::from([(row, col)]);

        loop {
            let new = cur
                .iter()
                .flat_map(|(ri, ci)| self.basinneighbours(*ri, *ci))
                .collect::<HashSet<(usize, usize)>>();
            if cur == new {
                break;
            } else {
                cur = new;
            }
        }
        cur
    }
}

fn parse_heightmap(lines: &[String]) -> HeightMap {
    HeightMap {
        heights: lines
            .iter()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|c| c - b'0')
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>(),
    }
}

fn p1(input: &[String]) -> u32 {
    parse_heightmap(input)
        .lowpoints()
        .iter()
        .map(|(_, _, n)| 1 + *n as u32)
        .sum::<u32>()
}

fn p2(input: &[String]) -> u32 {
    let heightmap = parse_heightmap(input);
    let mut sizes = heightmap
        .lowpoints()
        .iter()
        .map(|(row, col, _)| heightmap.basin(*row, *col).len())
        .collect::<Vec<usize>>();
    sizes.sort_unstable();
    sizes.iter().rev().collect::<Vec<&usize>>()[0..3]
        .iter()
        .fold(1, |prod, size| prod * (**size as u32))
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 9 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
