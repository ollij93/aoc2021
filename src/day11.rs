// Solutions for day11 of Advent of Code

use super::common::run_and_print_time;
use std::cmp::max;
use std::cmp::min;

#[derive(Clone)]
struct Cave {
    energies: Vec<Vec<u8>>,
}

impl Cave {
    fn process_flashes(&self) -> Cave {
        let flashers = self
            .energies
            .iter()
            .enumerate()
            .flat_map(|(ri, row)| row.iter().enumerate().map(move |(ci, x)| (ri, ci, x)))
            .filter(|(_, _, x)| **x > 9);
        Cave {
            energies: flashers.fold(self.energies.clone(), |mut energies, (ri, ci, _)| {
                // Increase the energy of all neighbours that haven't been reset
                for subrow in energies
                    .iter_mut()
                    .take(min(ri, 8) + 2)
                    .skip(max(ri, 1) - 1)
                {
                    for item in subrow.iter_mut().take(min(ci, 8) + 2).skip(max(ci, 1) - 1) {
                        if *item > 0 {
                            *item += 1;
                        }
                    }
                }
                // Reset the flashing energy to zero
                energies[ri][ci] = 0;
                energies
            }),
        }
    }
    fn step(&self) -> (u32, Cave) {
        let increased_energies = self
            .energies
            .iter()
            .map(|row| row.iter().map(|x| x + 1).collect())
            .collect();
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

fn p1(cave: Cave) -> u32 {
    let (ret, _) = (0..100).fold((0, cave), |(count, cave), _| {
        let (stepcount, newcave) = cave.step();
        (stepcount + count, newcave)
    });
    ret
}

fn p2(mut cave: Cave) -> u32 {
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

pub fn run(input: Vec<Vec<u8>>) -> u128 {
    println!("=== DAY 11 ===");

    let cave = Cave { energies: input };

    let (a, timea) = run_and_print_time(p1, cave.clone());
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, cave);
    println!("Part2: {}", b);

    timea + timeb
}
