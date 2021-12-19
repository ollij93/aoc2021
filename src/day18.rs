// Solutions for day18 of Advent of Code

use super::common::run_and_print_time;
use std::cmp::max;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone)]
enum SnailfishDigit {
    Digit(u32),
    Pair(Box<SnailfishDigit>, Box<SnailfishDigit>),
}
impl FromStr for SnailfishDigit {
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut stack = vec![];
        let mut first_digit: Option<SnailfishDigit> = None;
        let mut second_digit: Option<SnailfishDigit> = None;
        for c in s.chars() {
            match c {
                '[' => {
                    // Push our currently read context onto the stack
                    stack.push((first_digit, second_digit));
                    // Clear the existing state
                    first_digit = None;
                    second_digit = None;
                }
                ']' => {
                    let context = stack.pop().unwrap();
                    if context.0.is_none() {
                        first_digit = Some(SnailfishDigit::Pair(
                            Box::new(first_digit.unwrap()),
                            Box::new(second_digit.unwrap()),
                        ));
                        second_digit = None;
                    } else {
                        second_digit = Some(SnailfishDigit::Pair(
                            Box::new(first_digit.unwrap()),
                            Box::new(second_digit.unwrap()),
                        ));
                        first_digit = context.0;
                    }
                }
                '0'..='9' => {
                    let digit = SnailfishDigit::Digit(c.to_digit(10).unwrap());
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    } else {
                        second_digit = Some(digit);
                    }
                }
                ',' => (),
                x => panic!("Unexpected character: {}", x),
            }
        }
        Ok(first_digit.unwrap())
    }
    type Err = ();
}
impl Debug for SnailfishDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SnailfishDigit::Digit(d) => {
                write!(f, "{}", d)
            }
            SnailfishDigit::Pair(a, b) => write!(f, "[")
                .and(a.fmt(f))
                .and(write!(f, ","))
                .and(b.fmt(f))
                .and(write!(f, "]")),
        }
    }
}
impl SnailfishDigit {
    fn magnitude(&self) -> u32 {
        match self {
            SnailfishDigit::Digit(d) => *d,
            SnailfishDigit::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn add(&self, other: &Self) -> SnailfishDigit {
        let mut newpair = SnailfishDigit::Pair(Box::new(self.clone()), Box::new(other.clone()));
        while newpair.reduce() {}
        newpair
    }

    fn reduce(&mut self) -> bool {
        let (exp, _, _) = self.explode(0);
        let split = if !exp { self.split() } else { false };
        exp || split
    }

    fn explode(&mut self, depth: u32) -> (bool, Option<u32>, Option<u32>) {
        match self {
            SnailfishDigit::Digit(_) => panic!("Can't explode a digit"),
            SnailfishDigit::Pair(a, b) => match (*a.clone(), *b.clone()) {
                (SnailfishDigit::Digit(ad), SnailfishDigit::Digit(bd)) => {
                    if depth >= 4 {
                        *self = SnailfishDigit::Digit(0);
                        (true, Some(ad), Some(bd))
                    } else {
                        (false, None, None)
                    }
                }
                (SnailfishDigit::Digit(_), SnailfishDigit::Pair(_, _)) => {
                    let (exp, x, y) = b.explode(depth + 1);
                    if let Some(val) = x {
                        a.add_right(val)
                    }
                    (exp, None, y)
                }
                (SnailfishDigit::Pair(_, _), SnailfishDigit::Digit(_)) => {
                    let (exp, x, y) = a.explode(depth + 1);
                    if let Some(val) = y {
                        b.add_left(val)
                    }
                    (exp, x, None)
                }
                (SnailfishDigit::Pair(_, _), SnailfishDigit::Pair(_, _)) => {
                    let (exp, lx, ly) = a.explode(depth + 1);
                    if exp {
                        if let Some(val) = ly {
                            b.add_left(val)
                        }
                        (exp, lx, None)
                    } else {
                        let (expl, rx, ry) = b.explode(depth + 1);
                        if let Some(val) = rx {
                            a.add_right(val)
                        }
                        (expl, None, ry)
                    }
                }
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailfishDigit::Digit(d) => {
                if *d > 9 {
                    let left = *d / 2;
                    let right = if left * 2 == *d { left } else { left + 1 };
                    *self = SnailfishDigit::Pair(
                        Box::new(SnailfishDigit::Digit(left)),
                        Box::new(SnailfishDigit::Digit(right)),
                    );
                    true
                } else {
                    false
                }
            }
            SnailfishDigit::Pair(a, b) => a.split() || b.split(),
        }
    }

    fn add_left(&mut self, val: u32) {
        match self {
            SnailfishDigit::Digit(d) => *self = SnailfishDigit::Digit(*d + val),
            SnailfishDigit::Pair(a, _) => a.add_left(val),
        }
    }
    fn add_right(&mut self, val: u32) {
        match self {
            SnailfishDigit::Digit(d) => *self = SnailfishDigit::Digit(*d + val),
            SnailfishDigit::Pair(_, b) => b.add_right(val),
        }
    }
}

fn p1(input: &[String]) -> u32 {
    let numbers = input
        .iter()
        .filter_map(|s| match SnailfishDigit::from_str(s) {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<SnailfishDigit>>();

    numbers[1..]
        .iter()
        .fold(numbers[0].clone(), |sum, num| sum.add(num))
        .magnitude()
}

fn p2(input: &[String]) -> u32 {
    let numbers = input
        .iter()
        .filter_map(|s| match SnailfishDigit::from_str(s) {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<SnailfishDigit>>();

    numbers.iter().fold(0, |currmax, a| {
        max(
            currmax,
            numbers.iter().fold(0, |submax, b| {
                let mag = a.add(b).magnitude();
                max(submax, mag)
            }),
        )
    })
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 18 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            SnailfishDigit::from_str("[1,2]"),
            Ok(SnailfishDigit::Pair(
                Box::new(SnailfishDigit::Digit(1)),
                Box::new(SnailfishDigit::Digit(2))
            ))
        );
        assert_eq!(
            SnailfishDigit::from_str("[[1,2],3]"),
            Ok(SnailfishDigit::Pair(
                Box::new(SnailfishDigit::Pair(
                    Box::new(SnailfishDigit::Digit(1)),
                    Box::new(SnailfishDigit::Digit(2))
                )),
                Box::new(SnailfishDigit::Digit(3))
            ))
        );
        assert_eq!(
            SnailfishDigit::from_str("[9,[8,7]]"),
            Ok(SnailfishDigit::Pair(
                Box::new(SnailfishDigit::Digit(9)),
                Box::new(SnailfishDigit::Pair(
                    Box::new(SnailfishDigit::Digit(8)),
                    Box::new(SnailfishDigit::Digit(7))
                ))
            ))
        );
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            {
                let mut a = SnailfishDigit::from_str("[[[[[9,8],1],2],3],4]").unwrap();
                a.explode(0);
                a
            },
            SnailfishDigit::from_str("[[[[0,9],2],3],4]").unwrap()
        )
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            {
                let a = SnailfishDigit::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
                let b = SnailfishDigit::from_str("[1,1]").unwrap();
                a.add(&b)
            },
            SnailfishDigit::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()
        )
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            SnailfishDigit::from_str("[[1,2],[[3,4],5]]")
                .unwrap()
                .magnitude(),
            143
        );
        assert_eq!(
            SnailfishDigit::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
                .unwrap()
                .magnitude(),
            1384
        );
        assert_eq!(
            SnailfishDigit::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")
                .unwrap()
                .magnitude(),
            445
        );
        assert_eq!(
            SnailfishDigit::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]")
                .unwrap()
                .magnitude(),
            791
        );
        assert_eq!(
            SnailfishDigit::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")
                .unwrap()
                .magnitude(),
            1137
        );
        assert_eq!(
            SnailfishDigit::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .unwrap()
                .magnitude(),
            3488
        );
        assert_eq!(
            SnailfishDigit::from_str(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )
            .unwrap()
            .magnitude(),
            4140
        );
    }

    #[test]
    fn test_par1() {
        assert_eq!(
            p1(&[
                "[1,1]".to_string(),
                "[2,2]".to_string(),
                "[3,3]".to_string(),
                "[4,4]".to_string()
            ]),
            445
        )
    }
}
