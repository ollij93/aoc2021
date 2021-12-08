// Solutions for day8 of Advent of Code

use super::common::run_and_print_time;
use std::collections::HashSet;
use std::ops::Sub;

struct Signal {
    digits: [String; 10],
    codes: [String; 4],
}

fn get_chars_for_len<const N: usize>(arr: &[String; N], len: usize) -> HashSet<char> {
    arr.iter()
        .filter(|d| d.len() == len)
        .map(|s| s.chars().collect::<HashSet<char>>())
        .fold(
            "abcdefg".to_string().chars().collect::<HashSet<char>>(),
            |set, subset| {
                set.intersection(&subset)
                    .copied()
                    .collect::<HashSet<char>>()
            },
        )
}

impl Signal {
    fn to_u32(&self) -> u32 {
        let one = get_chars_for_len(&self.digits, 2);
        let four = get_chars_for_len(&self.digits, 4);
        let seven = get_chars_for_len(&self.digits, 3);
        let eight = get_chars_for_len(&self.digits, 7);

        let zero_six_nine = get_chars_for_len(&self.digits, 6);

        // A = 7 - 1
        let signala = seven.sub(&one);
        // B = (4 - 1) & 0 & 6 & 9
        let signalb = four
            .sub(&one)
            .intersection(&zero_six_nine)
            .copied()
            .collect::<HashSet<char>>();
        // D = (4 - 1) - B
        let signald = four.sub(&one).sub(&signalb);

        // F = 1 & 0 & 6 & 9
        let signalf = one
            .intersection(&zero_six_nine)
            .copied()
            .collect::<HashSet<char>>();
        // C = 1 - F
        let signalc = one.sub(&signalf);

        // G = (0 & 9 & 6) - A - B - F
        let signalg = zero_six_nine.sub(&signala).sub(&signalb).sub(&signalf);
        // E = 8 - <everything>
        let signale = eight
            .sub(&signala)
            .sub(&signalb)
            .sub(&signalc)
            .sub(&signald)
            .sub(&signalf)
            .sub(&signalg);

        let zero = eight.sub(&signald);
        let six = eight.sub(&signalc);
        let nine = eight.sub(&signale);
        let two = eight.sub(&signalb).sub(&signalf);

        let three = nine.sub(&signalb);
        let five = nine.sub(&signalc);

        self.codes.iter().fold(0, |value, code| {
            value * 10 + {
                let codeset = code.chars().collect::<HashSet<char>>();
                if codeset == one {
                    1
                } else if codeset == two {
                    2
                } else if codeset == three {
                    3
                } else if codeset == four {
                    4
                } else if codeset == five {
                    5
                } else if codeset == six {
                    6
                } else if codeset == seven {
                    7
                } else if codeset == eight {
                    8
                } else if codeset == nine {
                    9
                } else if codeset == zero {
                    0
                } else {
                    panic!("Failed to solve!")
                }
            }
        })
    }
}

fn parse_digits<const N: usize>(s: String) -> [String; N] {
    let (_, ret) = s.split(' ').collect::<Vec<&str>>()[0..N].iter().fold(
        (0, [""; N]),
        |(idx, mut digits), s| {
            digits[idx] = s;
            (idx + 1, digits)
        },
    );
    ret.map(|s| s.to_string())
}

fn parse_input(lines: &[String]) -> Vec<Signal> {
    let mut ret = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let signal = Signal {
            digits: parse_digits::<10>(parts[0].to_string()),
            codes: parse_digits::<4>(parts[1].to_string()),
        };
        ret.push(signal);
    }
    ret
}

fn p1(input: &[String]) -> u32 {
    let signals = parse_input(input);
    signals.iter().fold(0, |count, signal| {
        count
            + signal
                .codes
                .iter()
                .filter(|code| {
                    code.len() == 2 || code.len() == 3 || code.len() == 4 || code.len() == 7
                })
                .count()
    }) as u32
}

fn p2(input: &[String]) -> u32 {
    let signals = parse_input(input);
    signals.iter().map(|signal| signal.to_u32()).sum::<u32>()
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 8 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
