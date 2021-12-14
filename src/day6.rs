// Solutions for day6 of Advent of Code

use super::common::run_and_print_time;

struct FishSchool([u64; 9]);

fn create_school(input: &[u64]) -> FishSchool {
    FishSchool(input.iter().fold([0; 9], |mut school: [u64; 9], fish| {
        school[*fish as usize] += 1;
        school
    }))
}

fn simulate_day(school: &FishSchool) -> FishSchool {
    FishSchool((0..9).fold([0; 9], |mut ret: [u64; 9], day| {
        if day == 0 {
            ret[6] += school.0[0];
            ret[8] = school.0[0];
        } else {
            ret[day - 1] += school.0[day];
        }
        ret
    }))
}

fn p1(input: &[u64]) -> u64 {
    let init_school = create_school(input);
    let final_school = (0..80).fold(init_school, |fish, _| simulate_day(&fish));
    final_school.0.iter().sum()
}

fn p2(input: &[u64]) -> u64 {
    let init_school = create_school(input);
    let final_school = (0..256).fold(init_school, |fish, _| simulate_day(&fish));
    final_school.0.iter().sum()
}

pub fn run(input: Vec<u64>) -> u128 {
    println!("=== DAY 6 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}