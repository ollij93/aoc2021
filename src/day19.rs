// Solutions for day19 of Advent of Code

use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use super::common::run_and_print_time;
use crate::point::Point3;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Orientation {
    // Orientations as facing followed by up then the right direction is a given
    PosXPosY,
    PosXPosZ,
    PosXNegY,
    PosXNegZ,
    PosYPosX,
    PosYPosZ,
    PosYNegX,
    PosYNegZ,
    PosZPosX,
    PosZPosY,
    PosZNegX,
    PosZNegY,
    NegXPosY,
    NegXPosZ,
    NegXNegY,
    NegXNegZ,
    NegYPosX,
    NegYPosZ,
    NegYNegX,
    NegYNegZ,
    NegZPosX,
    NegZPosY,
    NegZNegX,
    NegZNegY,
}
impl Orientation {
    fn rel_to_world(&self, relpos: &Point3) -> Point3 {
        let (x, y, z) = (relpos.x, relpos.y, relpos.z);
        match self {
            // Facing along X
            Orientation::PosXPosY => Point3 { x, y, z },
            Orientation::PosXPosZ => Point3 { x, y: z, z: -y },
            Orientation::PosXNegY => Point3 { x, y: -y, z: -z },
            Orientation::PosXNegZ => Point3 { x, y: -z, z: y },
            // Facing along Y
            Orientation::PosYPosX => Point3 { x: y, y: x, z: -z },
            Orientation::PosYPosZ => Point3 { x: y, y: z, z: x },
            Orientation::PosYNegX => Point3 { x: y, y: -x, z },
            Orientation::PosYNegZ => Point3 { x: y, y: -z, z: -x },
            // Facing along Z
            Orientation::PosZPosY => Point3 { x: z, y, z: -x },
            Orientation::PosZPosX => Point3 { x: z, y: x, z: y },
            Orientation::PosZNegY => Point3 { x: z, y: -y, z: x },
            Orientation::PosZNegX => Point3 { x: z, y: -x, z: -y },
            // Facing along negative X
            Orientation::NegXPosY => Point3 { x: -x, y, z: -z },
            Orientation::NegXPosZ => Point3 { x: -x, y: z, z: y },
            Orientation::NegXNegY => Point3 { x: -x, y: -y, z },
            Orientation::NegXNegZ => Point3 {
                x: -x,
                y: -z,
                z: -y,
            },
            // Facing along Y
            Orientation::NegYPosX => Point3 { x: -y, y: x, z },
            Orientation::NegYPosZ => Point3 { x: -y, y: z, z: -x },
            Orientation::NegYNegX => Point3 {
                x: -y,
                y: -x,
                z: -z,
            },
            Orientation::NegYNegZ => Point3 { x: -y, y: -z, z: x },
            // Facing along Z
            Orientation::NegZPosY => Point3 { x: -z, y, z: x },
            Orientation::NegZPosX => Point3 { x: -z, y: x, z: -y },
            Orientation::NegZNegY => Point3 {
                x: -z,
                y: -y,
                z: -x,
            },
            Orientation::NegZNegX => Point3 { x: -z, y: -x, z: y },
        }
    }
}

fn all_orientations(relpos: &Point3) -> Vec<PosOri> {
    vec![
        Orientation::PosXPosY,
        Orientation::PosXPosZ,
        Orientation::PosXNegY,
        Orientation::PosXNegZ,
        Orientation::PosYPosX,
        Orientation::PosYPosZ,
        Orientation::PosYNegX,
        Orientation::PosYNegZ,
        Orientation::PosZPosX,
        Orientation::PosZPosY,
        Orientation::PosZNegX,
        Orientation::PosZNegY,
        Orientation::NegXPosY,
        Orientation::NegXPosZ,
        Orientation::NegXNegY,
        Orientation::NegXNegZ,
        Orientation::NegYPosX,
        Orientation::NegYPosZ,
        Orientation::NegYNegX,
        Orientation::NegYNegZ,
        Orientation::NegZPosX,
        Orientation::NegZPosY,
        Orientation::NegZNegX,
        Orientation::NegZNegY,
    ]
    .iter()
    .map(|ori| PosOri {
        pos: ori.rel_to_world(relpos),
        ori: *ori,
    })
    .collect()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct PosOri {
    pos: Point3,
    ori: Orientation,
}

#[derive(Clone, Eq, PartialEq)]
struct Scanner {
    idx: u32,
    beacons: HashSet<Point3>,
    known_position: Option<PosOri>,
}
impl Scanner {
    fn new(idx: u32) -> Scanner {
        Scanner {
            idx,
            beacons: HashSet::new(),
            known_position: None,
        }
    }

    fn is_valid_pos(&self, pos_ori: &PosOri, known_beacons: &HashSet<Point3>) -> bool {
        let beacons: HashSet<Point3> = self
            .beacons
            .iter()
            .map(|p| pos_ori.ori.rel_to_world(p) + &pos_ori.pos)
            .collect();
        let agreed_beacons: HashSet<&Point3> = beacons.intersection(known_beacons).collect();
        // Need at least 12 beacons to overlap for this to be a confirmed position
        agreed_beacons.len() >= 12
    }

    /// Find all the possible positions of this scanner given the well known
    /// positions of beacons that exist
    fn find_valid_positions(&self, known_beacons: &HashSet<Point3>) -> HashSet<PosOri> {
        // Possible positions to try are those where the first point aligns
        // with the first in the other, the second in the other, the third in
        // the other, etc and where the second aligns with the same, and so on.
        let all_positions = known_beacons
            .iter()
            .flat_map(|world_beacon| {
                self.beacons.iter().flat_map(|rel_beacon| {
                    // Given a world position and a relative position for
                    // this beacon, figure out what positions we could be
                    // in if we're looking in each possible direction.

                    // Possible positions of the beacon relative to the scanner
                    // in world orientation and the orientation the scanner
                    // would have for this to be true
                    all_orientations(rel_beacon)
                        .iter()
                        .map(|beacon_pos_ori| PosOri {
                            pos: *world_beacon - beacon_pos_ori.pos,
                            ori: beacon_pos_ori.ori,
                        })
                        .collect::<Vec<PosOri>>()
                })
            })
            .collect::<HashSet<PosOri>>();
        all_positions
            .iter()
            .filter(|pos| self.is_valid_pos(pos, known_beacons))
            .cloned()
            .collect::<HashSet<PosOri>>()
    }
}

fn parse_input(input: &[String]) -> HashMap<u32, Scanner> {
    let mut scanners: HashMap<u32, Scanner> = HashMap::new();
    let mut scanner: Scanner = Scanner {
        idx: 0,
        beacons: HashSet::new(),
        known_position: Some(PosOri {
            pos: Point3 { x: 0, y: 0, z: 0 },
            ori: Orientation::PosXPosY,
        }),
    };

    for line in input {
        if line.is_empty() {
            continue;
        }
        match Point3::from_str(line) {
            Ok(val) => {
                scanner.beacons.insert(val);
            }
            Err(_) => {
                if !scanner.beacons.is_empty() {
                    scanners.insert(scanner.idx, scanner);
                    let idx: u32 = scanners.len() as u32;
                    scanner = Scanner::new(idx);
                }
            }
        }
    }

    scanners.insert(scanner.idx, scanner);
    scanners
}
fn manhattan_distance(a: &Point3, b: &Point3) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs() + (b.z - a.z).abs()
}

fn calculate_manhattans(points: &HashSet<Point3>) -> HashSet<i32> {
    let pointsvec = points.iter().collect::<Vec<&Point3>>();
    (0..points.len())
        .map(|idx1| {
            (idx1 + 1..points.len())
                .map(|idx2| manhattan_distance(pointsvec[idx1], pointsvec[idx2]))
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<HashSet<i32>>()
}

fn calculate_beacon_manhattans(scanners: &HashMap<u32, Scanner>) -> HashMap<u32, HashSet<i32>> {
    scanners
        .iter()
        .map(|(k, v)| (*k, calculate_manhattans(&v.beacons)))
        .collect::<HashMap<u32, HashSet<i32>>>()
}

fn get_all_scanners(input: &[String]) -> HashMap<u32, Scanner> {
    let mut scanners = parse_input(input);
    let manhattans = calculate_beacon_manhattans(&scanners);
    let mut unknown_idxs: Vec<u32> = scanners.keys().filter(|k| **k > 0).copied().collect();
    let mut known_unchecked: Vec<u32> = vec![0];
    let mut known_checked = vec![];

    while let Some(known_idx) = known_unchecked.pop() {
        let newly_known: Vec<(u32, PosOri)> = unknown_idxs
            .iter()
            .filter_map(|unknown_idx| {
                let known_scanner = &scanners[&known_idx];
                let unknown_scanner = &scanners[unknown_idx];
                let positions = match known_scanner.known_position {
                    None => panic!("Don't know the position of a known scanner!"),
                    Some(pos_ori) => {
                        let known_manhattan = &manhattans[&known_idx];
                        let unknown_manhattan = &manhattans[unknown_idx];
                        if known_manhattan
                            .intersection(unknown_manhattan)
                            .collect::<HashSet<&i32>>()
                            .len()
                            < 12
                        {
                            HashSet::new()
                        } else {
                            unknown_scanner.find_valid_positions(
                                &known_scanner
                                    .beacons
                                    .iter()
                                    .map(|p| pos_ori.ori.rel_to_world(p) + &pos_ori.pos)
                                    .collect::<HashSet<Point3>>(),
                            )
                        }
                    }
                };
                if positions.len() == 1 {
                    let pos_ori = positions.iter().cloned().collect::<Vec<PosOri>>()[0];
                    Some((unknown_scanner.idx, pos_ori))
                } else {
                    None
                }
            })
            .collect();

        unknown_idxs = unknown_idxs
            .iter()
            .filter(|idx| !newly_known.iter().any(|(x, _)| *x == **idx))
            .copied()
            .collect();

        for (idx, posori) in newly_known {
            known_unchecked.push(idx);
            scanners.insert(
                idx,
                Scanner {
                    idx,
                    beacons: scanners[&idx].beacons.clone(),
                    known_position: Some(posori),
                },
            );
        }

        known_checked.push(known_idx);
    }
    scanners
}

fn p1(scanners: &HashMap<u32, Scanner>) -> u32 {
    let all_beacons = scanners
        .keys()
        .flat_map(|idx| match scanners[idx].known_position {
            None => panic!(),
            Some(pos_ori) => scanners[idx]
                .beacons
                .iter()
                .map(move |p| pos_ori.ori.rel_to_world(p) + &pos_ori.pos),
        })
        .collect::<HashSet<Point3>>();
    all_beacons.len() as u32
}

fn p2(scanners: &HashMap<u32, Scanner>) -> i32 {
    scanners.keys().fold(0, |mx, scanner_a| {
        max(
            mx,
            scanners.keys().fold(0, |submx, scanner_b| {
                max(
                    submx,
                    manhattan_distance(
                        &scanners[scanner_a].known_position.unwrap().pos,
                        &scanners[scanner_b].known_position.unwrap().pos,
                    ),
                )
            }),
        )
    })
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 19 ===");

    let (scanners, time_scanners) = run_and_print_time(get_all_scanners, &input);
    println!("Got scanners.");

    let (a, timea) = run_and_print_time(p1, &scanners);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &scanners);
    println!("Part2: {}", b);

    time_scanners + timea + timeb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_orientations() {
        assert_eq!(
            all_orientations(&Point3 { x: 1, y: 2, z: 3 })
                .iter()
                .map(|wpo| wpo.pos)
                .collect::<HashSet<Point3>>()
                .len(),
            24
        )
    }
}
