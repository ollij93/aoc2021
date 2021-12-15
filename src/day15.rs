// Solutions for day15 of Advent of Code

use super::common::run_and_print_time;
use super::point::Point;
use std::cmp::Reverse;

use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
struct Vertex(Point, u32);
impl std::cmp::PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

fn get_neighbors(point: &Point, size: (u32, u32)) -> Vec<Point> {
    let mut ret = vec![];
    // Left
    if point.x > 1 {
        ret.push(Point {
            x: point.x - 1,
            y: point.y,
        });
    }
    // Right
    if point.x < size.0 - 1 {
        ret.push(Point {
            x: point.x + 1,
            y: point.y,
        });
    }
    // Up
    if point.y > 1 {
        ret.push(Point {
            x: point.x,
            y: point.y - 1,
        });
    }
    // Down
    if point.y < size.1 - 1 {
        ret.push(Point {
            x: point.x,
            y: point.y + 1,
        });
    }
    ret
}

fn dijkstra(nodecosts: &[Vec<u8>], start: Point) -> HashMap<Point, u32> {
    let mut costs: HashMap<Point, u32> = HashMap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit: BinaryHeap<Reverse<Vertex>> = BinaryHeap::new();

    costs.insert(start.clone(), 0);
    to_visit.push(Reverse(Vertex(start, 0)));

    while let Some(Reverse(Vertex(point, cost))) = to_visit.pop() {
        if !visited.insert(point.clone()) {
            // Already visited this node
            continue;
        }

        for neighbor in get_neighbors(&point, (nodecosts[0].len() as u32, nodecosts.len() as u32)) {
            let nodecost = nodecosts[neighbor.y as usize][neighbor.x as usize];
            let new_cost = cost + nodecost as u32;
            let is_shorter = costs
                .get(&neighbor)
                .map_or(true, |&current| new_cost < current);

            if is_shorter {
                costs.insert(neighbor.clone(), new_cost);
                to_visit.push(Reverse(Vertex(neighbor, new_cost)));
            }
        }
    }

    costs
}

fn p1(input: &[Vec<u8>]) -> u32 {
    let paths = dijkstra(input, Point { x: 0, y: 0 });
    paths[&Point {
        x: input[0].len() as u32 - 1,
        y: input.len() as u32 - 1,
    }]
}

fn p2(single_cave: &[Vec<u8>]) -> u32 {
    let stretched_x: Vec<Vec<u8>> = single_cave
        .iter()
        .map(|row| {
            (0..5).fold(vec![], |line, _| {
                row.iter()
                    .copied()
                    .chain(line.iter().map(|x| (x % 9) + 1))
                    .collect::<Vec<u8>>()
            })
        })
        .collect();

    let full_cave: Vec<Vec<u8>> = (0..5).fold(vec![], |cave, _| {
        stretched_x
            .iter()
            .chain(
                &cave
                    .iter()
                    .map(|row| row.iter().map(|x| (x % 9) + 1).collect::<Vec<u8>>())
                    .collect::<Vec<Vec<u8>>>(),
            )
            .map(|x| x.to_owned())
            .collect::<Vec<Vec<u8>>>()
    });
    let paths = dijkstra(&full_cave, Point { x: 0, y: 0 });
    paths[&Point {
        x: full_cave[0].len() as u32 - 1,
        y: full_cave.len() as u32 - 1,
    }]
}

pub fn run(input: Vec<Vec<u8>>) -> u128 {
    println!("=== DAY 15 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
