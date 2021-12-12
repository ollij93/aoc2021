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

#[derive(Clone)]
enum Route<'a> {
    Start,
    Path {
        curr: &'a Cave,
        previous: Box<Route<'a>>,
    },
}

impl<'a> Route<'a> {
    fn contains(&self, cave: &'a Cave) -> bool {
        match self {
            Route::Start => false,
            Route::Path { curr, previous } => **curr == *cave || previous.contains(cave),
        }
    }
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

fn get_routes<'a>(
    cave_system: &'a HashMap<Cave, HashSet<Cave>>,
    route: Route<'a>,
    allowed_small_revisits: u32,
) -> Vec<Route<'a>> {
    // Can't figure out how to make these constants due to the use of string
    let start_cave = Cave {
        name: "start".to_string(),
        size: CaveSize::Small,
    };
    let end_cave = Cave {
        name: "end".to_string(),
        size: CaveSize::Small,
    };

    let current = match route {
        Route::Path { curr, previous: _ } => curr,
        Route::Start => &start_cave,
    };
    // If this is the end cave, return the route here as a successful route
    if *current == end_cave {
        return vec![route];
    }

    // Visit all the next caves except those that aren't permitted
    cave_system[current]
        .iter()
        .flat_map(|cave| {
            let is_small_revisit = cave.size == CaveSize::Small && route.contains(cave);
            if cave.name == "start" || (allowed_small_revisits == 0 && is_small_revisit) {
                vec![]
            } else {
                let sub_allowed_small_revisits = if is_small_revisit {
                    allowed_small_revisits - 1
                } else {
                    allowed_small_revisits
                };

                let new_path = Route::Path {
                    curr: cave,
                    previous: Box::new(route.clone()), // AHHHH! HOW TO AVOID THIS CLONE?
                };

                get_routes(cave_system, new_path, sub_allowed_small_revisits)
            }
        })
        .collect::<Vec<Route<'a>>>()
}

fn p1(input: &[String]) -> u32 {
    let cave_system = parse_cave_system(input);

    get_routes(&cave_system, Route::Start, 0).len() as u32
}

fn p2(input: &[String]) -> u32 {
    let cave_system = parse_cave_system(input);

    get_routes(&cave_system, Route::Start, 1).len() as u32
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 12 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
