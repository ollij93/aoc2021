// Solutions for day3 of Advent of Code

use super::common::run_and_print_time;

fn str_to_binary(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect::<Vec<bool>>()
}

fn binary_to_u32(input: &[bool]) -> u32 {
    input.iter().fold(0, |ret, cur| ret * 2 + (*cur as u32))
}

fn count_on_bits(input: &[Vec<bool>]) -> Vec<u32> {
    input[1..].iter().fold(
        // Initialise using the first element to determine the size of vector we need
        input[0].iter().map(|b| *b as u32).collect(),
        // Add onto the existing counts for each binary vector we get
        |counts, vec| counts.iter().zip(vec).map(|(c, b)| c + *b as u32).collect(),
    )
}

fn get_majorities(input: &[u32], size: u32) -> Vec<bool> {
    input.iter().map(|x| x * 2 >= size).collect::<Vec<bool>>()
}

fn p1(input: &[String]) -> u32 {
    let binary_input = &input
        .iter()
        .map(|s| str_to_binary(s))
        .collect::<Vec<Vec<bool>>>();
    let counts = count_on_bits(binary_input);
    let majorities = get_majorities(&counts, input.len() as u32);
    let gamma = binary_to_u32(&majorities);
    let epsilon = binary_to_u32(&majorities.iter().map(|x| !x).collect::<Vec<bool>>());
    gamma * epsilon
}

fn filter_on_majority<F>(possibles: Vec<Vec<bool>>, fltr: F, size: usize) -> Vec<bool>
where
    F: Fn(bool, bool) -> bool,
{
    let ret = (0..size).fold(possibles, |pos, i| {
        if pos.len() == 1 {
            // Only one possible result, so keep that
            pos
        } else {
            let counts = count_on_bits(&pos);
            let majorities = get_majorities(&counts, pos.len() as u32);
            pos.iter()
                .filter(|x| fltr(x[i], majorities[i]))
                .map(|x| x.to_owned())
                .collect::<Vec<Vec<bool>>>()
        }
    });
    ret[0].to_owned()
}

fn p2(input: &[String]) -> u32 {
    let size = input[0].len();
    let binary_input: Vec<Vec<bool>> = input.iter().map(|s| str_to_binary(s)).collect();
    let oxygen = binary_to_u32(&filter_on_majority(
        binary_input.to_owned(),
        |b, maj| b == maj,
        size,
    ));
    let co2 = binary_to_u32(&filter_on_majority(binary_input, |b, maj| b != maj, size));

    oxygen * co2
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 3 ===");

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
    fn test_count_on_bits() {
        let input: Vec<Vec<bool>> = vec![
            vec![false, false, false],
            vec![true, false, false],
            vec![true, true, false],
            vec![true, true, false],
            vec![true, true, false],
            vec![false, false, true],
        ];
        assert_eq!(count_on_bits(&input), vec![4, 3, 1]);
    }

    #[test]
    fn test_get_majorities() {
        assert_eq!(get_majorities(&[], 10), vec![]);
        assert_eq!(get_majorities(&[1, 50, 20], 50), vec![false, true, false]);
        assert_eq!(get_majorities(&[1], 2), vec![true]);
        assert_eq!(get_majorities(&[1], 3), vec![false])
    }
}
