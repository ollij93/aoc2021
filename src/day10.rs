// Solutions for day10 of Advent of Code

use super::common::{median, run_and_print_time};

#[derive(Debug)]
enum Score {
    SyntaxChecker(u32),
    AutoCorrect(u64),
    Zero,
}

fn parse_line(line: &str) -> Score {
    let mut stack: Vec<u8> = vec![0];
    for c in line.chars() {
        if ['(', '[', '{', '<'].contains(&c) {
            stack.push(c as u8)
        } else {
            match (stack.last().unwrap(), c) {
                (b'(', ')') => {
                    stack.pop();
                }
                (b'[', ']') => {
                    stack.pop();
                }
                (b'{', '}') => {
                    stack.pop();
                }
                (b'<', '>') => {
                    stack.pop();
                }
                (_, ')') => {
                    return Score::SyntaxChecker(3);
                }
                (_, ']') => {
                    return Score::SyntaxChecker(57);
                }
                (_, '}') => {
                    return Score::SyntaxChecker(1197);
                }
                (_, '>') => {
                    return Score::SyntaxChecker(25137);
                }
                _ => panic!("Unexpected character in input: {}", c),
            }
        }
    }
    let acscore: u64 = stack[1..].iter().rev().fold(0, |score, c| {
        let charscore = match c {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => panic!("Unexpected character in stack: {}", c),
        };
        score * 5 + charscore
    });
    if acscore > 0 {
        Score::AutoCorrect(acscore)
    } else {
        Score::Zero
    }
}

fn p1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| parse_line(line))
        .map(|score| match score {
            Score::SyntaxChecker(val) => val,
            _ => 0,
        })
        .sum::<u32>()
}

fn p2(input: &[String]) -> u64 {
    median(
        &input
            .iter()
            .map(|line| parse_line(line))
            .map(|score| match score {
                Score::AutoCorrect(val) => val,
                _ => 0,
            })
            .filter(|x| *x > 0)
            .collect::<Vec<u64>>(),
    )
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 10 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
