// Solutions for day12 of Advent of Code

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use super::common::run_and_print_time;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum CaveSize {
    Small,
    Large,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Cave {
    name: String,
    size: CaveSize,
}

impl FromStr for Cave {
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Cave {
            name: s.to_string(),
            size: if s.to_uppercase() == s {
                CaveSize::Large
            } else {
                CaveSize::Small
            },
        })
    }
    type Err = String;
}

fn parse_connection(line: &str) -> (Cave, Cave) {
    let (name_a, name_b) = line.split_once("-").unwrap();
    (
        Cave::from_str(name_a).unwrap(),
        Cave::from_str(name_b).unwrap(),
    )
}

fn parse_cave_system(input: &[String]) -> HashMap<Cave, HashSet<Cave>> {
    input.iter().fold(HashMap::new(), |mut hm, line| {
        let (a, b) = parse_connection(line);
        hm.entry(a.to_owned())
            .or_insert_with(HashSet::new)
            .insert(b.to_owned());
        hm.entry(b).or_insert_with(HashSet::new).insert(a);
        hm
    })
}

fn get_routes(
    cave_system: &HashMap<Cave, HashSet<Cave>>,
    route: Vec<Cave>,
    allowed_small_revisits: u32,
) -> Vec<Vec<Cave>> {
    // Can't figure out how to make these constants due to the use of string
    let end_cave = Cave {
        name: "end".to_string(),
        size: CaveSize::Small,
    };

    let current = route.last().unwrap();
    // If this is the end cave, return the route here as a successful route
    if *current == end_cave {
        return vec![route];
    }

    // Visit all the next caves except those that aren't permitted
    let nexts = cave_system[current].iter().filter(|cave| {
        !((cave.name == "start")
            || cave.size == CaveSize::Small && route.contains(cave) && allowed_small_revisits == 0)
    });

    let mut ret = vec![];
    for next in nexts {
        let sub_allowed_small_revisits = if route.contains(next) && next.size == CaveSize::Small {
            allowed_small_revisits - 1
        } else {
            allowed_small_revisits
        };
        let mut next_route: Vec<Cave> = route.clone();
        next_route.push(next.to_owned());
        ret.extend(get_routes(
            cave_system,
            next_route,
            sub_allowed_small_revisits,
        ));
    }
    ret
}

fn p1(input: &[String]) -> u32 {
    let cave_system = parse_cave_system(input);
    let start_cave = Cave {
        name: "start".to_string(),
        size: CaveSize::Small,
    };

    get_routes(&cave_system, vec![start_cave], 0).len() as u32
}

fn p2(input: &[String]) -> u32 {
    let cave_system = parse_cave_system(input);
    let start_cave = Cave {
        name: "start".to_string(),
        size: CaveSize::Small,
    };

    get_routes(&cave_system, vec![start_cave], 1).len() as u32
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 12 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
