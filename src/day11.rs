// Solutions for day11 of Advent of Code

use super::common::run_and_print_time;
use std::cmp::max;
use std::cmp::min;

struct Cave {
    energies: [[u32; 10]; 10],
}

impl Cave {
    fn process_flashes(&self) -> Cave {
        let flashers = self
            .energies
            .iter()
            .enumerate()
            .flat_map(|(ri, row)| row.iter().enumerate().map(move |(ci, x)| (ri, ci, x)))
            .filter(|(_, _, x)| **x > 9);
        let ret = Cave {
            energies: flashers.fold(self.energies, |mut energies, (ri, ci, _)| {
                // Increase the energy of all neighbours that haven't been reset
                for subrow in energies
                    .iter_mut()
                    .take(min(ri, 8) + 2)
                    .skip(max(ri, 1) - 1)
                {
                    for item in subrow
                        .iter_mut()
                        .take(min(ci, 8) + 2)
                        .skip(max(ci, 1) - 1)
                    {
                        if *item > 0 {
                            *item += 1;
                        }
                    }
                }
                // Reset the flashing energy to zero
                energies[ri][ci] = 0;
                energies
            }),
        };
        ret
    }
    fn step(&self) -> (u32, Cave) {
        let increased_energies = self.energies.map(|row| row.map(|x| x + 1));
        let mut newcave = Cave {
            energies: increased_energies,
        };
        loop {
            let flashed_cave = newcave.process_flashes();
            if flashed_cave.energies == newcave.energies {
                break;
            } else {
                newcave = flashed_cave;
            }
        }
        let count = newcave
            .energies
            .iter()
            .flatten()
            .filter(|x| **x == 0)
            .count();
        (count as u32, newcave)
    }
}

fn parse_cave(input: &[String]) -> Cave {
    let mut cave: Cave = Cave {
        energies: [[0; 10]; 10],
    };
    for (row, value) in input.iter().enumerate() {
        for (col, c) in value.chars().enumerate() {
            cave.energies[row][col] = (c as u8 - b'0') as u32;
        }
    }
    cave
}

fn p1(input: &[String]) -> u32 {
    let cave = parse_cave(input);
    let (ret, _) = (0..100).fold((0, cave), |(count, cave), _| {
        let (stepcount, newcave) = cave.step();
        (stepcount + count, newcave)
    });
    ret
}

fn p2(input: &[String]) -> u32 {
    let mut cave = parse_cave(input);
    let mut step = 0;
    loop {
        if cave.energies == [[0; 10]; 10] {
            break;
        }
        step += 1;
        cave = cave.step().1;
    }
    step
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 11 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
