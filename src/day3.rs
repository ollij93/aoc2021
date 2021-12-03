// Solutions for day3 of Advent of Code

use super::common::run_and_print_time;

fn str_to_binary(input: &String) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect::<Vec<bool>>()
}

fn binary_to_u32(input: &Vec<bool>) -> u32 {
    let mut ret = 0;
    for b in input {
        ret = ret * 2;
        if *b {
            ret += 1;
        }
    }
    return ret;
}

fn count_on_bits(input: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();

    for vec in input {
        // Make sure our vector is long enough for this length of input
        while ret.len() < vec.len() {
            ret.push(0);
        }

        for i in 0..vec.len() {
            ret[i] += vec[i] as u32;
        }
    }

    return ret;
}

fn get_majorities(input: &Vec<u32>, size: u32) -> Vec<bool> {
    input.iter().map(|x| x * 2 >= size).collect::<Vec<bool>>()
}

fn p1(input: &Vec<String>) -> u32 {
    let binary_input = &input.iter().map(|s| str_to_binary(s)).collect();
    let counts = count_on_bits(binary_input);
    let majorities = get_majorities(&counts, (input.len() as u32) / 2);
    let gamma = binary_to_u32(&majorities);
    let epsilon = binary_to_u32(&majorities.iter().map(|x| !x).collect::<Vec<bool>>());

    println!("{:?} {:?}", gamma, epsilon);

    return gamma * epsilon;
}

fn p2(input: &Vec<String>) -> u32 {
    let binary_input: Vec<Vec<bool>> = input.iter().map(|s| str_to_binary(s)).collect();
    let mut oxygen_possibles = binary_input.to_owned();

    let mut i = 0;
    while oxygen_possibles.len() > 1 {
        let counts = count_on_bits(&oxygen_possibles);
        let majorities = get_majorities(&counts, oxygen_possibles.len() as u32);
        let v = majorities[i];
        oxygen_possibles = oxygen_possibles
            .iter()
            .filter(|x| x[i] == v)
            .map(|x| x.to_owned())
            .collect();
        i += 1;
    }

    let mut c02_possibles = binary_input.to_owned();
    i = 0;
    while c02_possibles.len() > 1 {
        let counts = count_on_bits(&c02_possibles);
        let majorities = get_majorities(&counts, c02_possibles.len() as u32);
        let v = majorities[i];
        c02_possibles = c02_possibles
            .iter()
            .filter(|x| x[i] != v)
            .map(|x| x.to_owned())
            .collect();
        i += 1;
    }

    let oxygen = binary_to_u32(&oxygen_possibles[0]);
    let c02 = binary_to_u32(&c02_possibles[0]);

    return oxygen * c02;
}

pub fn run(input: Vec<String>) {
    let a = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, &input);
    println!("Part2: {}", b);
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
        assert_eq!(get_majorities(&vec![], 10), vec![]);
        assert_eq!(
            get_majorities(&vec![1, 50, 20], 50),
            vec![false, true, false]
        );
        assert_eq!(get_majorities(&vec![1], 2), vec![true]);
        assert_eq!(get_majorities(&vec![1], 3), vec![false])
    }
}
