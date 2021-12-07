// Solutions for day7 of Advent of Code

use super::common::run_and_print_time;
use std::collections::HashMap;

fn calculate_cost<F>(map: &HashMap<u32, u32>, pos: u32, cost_to_move: &F) -> u32
where
    F: Fn(u32) -> u32,
{
    map.keys().fold(0, |cost, key| {
        if *key < pos {
            cost + map[key] * cost_to_move(pos - key)
        } else {
            cost + map[key] * cost_to_move(key - pos)
        }
    })
}

fn calculate_costs<F>(map: HashMap<u32, u32>, cost_to_move: &F) -> Vec<(u32, u32)>
where
    F: Fn(u32) -> u32,
{
    let max = map
        .keys()
        .fold(0, |mx, key| if *key > mx { *key } else { mx });
    (0..=max).fold(vec![], |mut ret, pos| {
        ret.push((pos, calculate_cost(&map, pos, &cost_to_move)));
        ret
    })
}

fn calculate_min_cost<F>(input: Vec<u32>, cost_to_move: &F) -> u32
where
    F: Fn(u32) -> u32,
{
    let costs = calculate_costs(
        input.iter().fold(HashMap::new(), |mut hashmap, pos| {
            *hashmap.entry(*pos).or_insert(0) += 1;
            hashmap
        }),
        cost_to_move,
    );
    costs.iter().fold(
        costs[0],
        |min, cost| {
            if cost.1 < min.1 {
                *cost
            } else {
                min
            }
        },
    ).1
}

fn p1(input: Vec<u32>) -> u32 {
    calculate_min_cost(input, &|x| x)
}

fn p2_cost_to_move(units: u32) -> u32 {
    if units == 0 {
        0
    } else {
        units + p2_cost_to_move(units - 1)
    }
}

fn p2(input: Vec<u32>) -> u32 {
    calculate_min_cost(input, &p2_cost_to_move)
}

pub fn run(input: Vec<u32>) {
    let a = run_and_print_time(p1, input.clone());
    println!("Part1: {}", a);

    let b = run_and_print_time(p2, input);
    println!("Part2: {}", b);
}
