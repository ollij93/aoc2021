// Solutions for day22 of Advent of Code

use super::common::run_and_print_time;
use std::cmp::max;
use std::cmp::min;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::str::FromStr;

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Region {
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
    minz: i64,
    maxz: i64,
}
impl FromStr for Region {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let (xstr, yzstr) = s.split_once(",").unwrap();
        let (ystr, zstr) = yzstr.split_once(",").unwrap();
        let (_, xrng) = xstr.split_once("=").unwrap();
        let (_, yrng) = ystr.split_once("=").unwrap();
        let (_, zrng) = zstr.split_once("=").unwrap();
        let (minx, maxx) = xrng.split_once("..").unwrap();
        let (miny, maxy) = yrng.split_once("..").unwrap();
        let (minz, maxz) = zrng.split_once("..").unwrap();
        Ok(Region {
            minx: minx.parse::<i64>().unwrap(),
            maxx: maxx.parse::<i64>().unwrap(),
            miny: miny.parse::<i64>().unwrap(),
            maxy: maxy.parse::<i64>().unwrap(),
            minz: minz.parse::<i64>().unwrap(),
            maxz: maxz.parse::<i64>().unwrap(),
        })
    }
}
impl Debug for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.minx, self.maxx, self.miny, self.maxy, self.minz, self.maxz
        )
    }
}
impl Region {
    fn size(&self) -> i64 {
        (self.maxx - self.minx + 1) * (self.maxy - self.miny + 1) * (self.maxz - self.minz + 1)
    }

    fn contains(&self, other: &Region) -> bool {
        self.minx <= other.minx
            && self.maxx >= other.maxx
            && self.miny <= other.miny
            && self.maxy >= other.maxy
            && self.minz <= other.minz
            && self.maxz >= other.maxz
    }

    fn splitx(self, x: i64) -> [Option<Region>; 2] {
        if x <= self.minx || x > self.maxx {
            [Some(self), None]
        } else {
            let lessregion = Region {
                minx: self.minx,
                maxx: x - 1,
                miny: self.miny,
                maxy: self.maxy,
                minz: self.minz,
                maxz: self.maxz,
            };
            let greaterregion = Region {
                minx: x,
                maxx: self.maxx,
                miny: self.miny,
                maxy: self.maxy,
                minz: self.minz,
                maxz: self.maxz,
            };
            [Some(lessregion), Some(greaterregion)]
        }
    }

    fn splity(self, y: i64) -> [Option<Region>; 2] {
        if y <= self.miny || y > self.maxy {
            [Some(self), None]
        } else {
            let lessregion = Region {
                minx: self.minx,
                maxx: self.maxx,
                miny: self.miny,
                maxy: y - 1,
                minz: self.minz,
                maxz: self.maxz,
            };
            let greaterregion = Region {
                minx: self.minx,
                maxx: self.maxx,
                miny: y,
                maxy: self.maxy,
                minz: self.minz,
                maxz: self.maxz,
            };
            [Some(lessregion), Some(greaterregion)]
        }
    }

    fn splitz(self, z: i64) -> [Option<Region>; 2] {
        if z <= self.minz || z > self.maxz {
            [Some(self), None]
        } else {
            let lessregion = Region {
                minx: self.minx,
                maxx: self.maxx,
                miny: self.miny,
                maxy: self.maxy,
                minz: self.minz,
                maxz: z - 1,
            };
            let greaterregion = Region {
                minx: self.minx,
                maxx: self.maxx,
                miny: self.miny,
                maxy: self.maxy,
                minz: z,
                maxz: self.maxz,
            };
            [Some(lessregion), Some(greaterregion)]
        }
    }

    fn overlap(self, other: &Region) -> Option<Region> {
        let region = Region {
            minx: max(self.minx, other.minx),
            maxx: min(self.maxx, other.maxx),
            miny: max(self.miny, other.miny),
            maxy: min(self.maxy, other.maxy),
            minz: max(self.minz, other.minz),
            maxz: min(self.maxz, other.maxz),
        };
        if region.minx > region.maxx || region.miny > region.maxy || region.minz > region.maxz {
            None
        } else {
            Some(region)
        }
    }

    fn merge_around(&self, other: &Region) -> HashSet<Region> {
        match self.overlap(other) {
            // No overlap, keep the whole region
            None => HashSet::from([*self]),
            Some(_) => {
                if other.contains(self) {
                    // This region completely contained within the other.
                    // Nothing to return.
                    HashSet::new()
                } else {
                    // Some overlap, keep the other region and split this region
                    // along all the boundaries
                    let splitregs = [self]
                        .iter()
                        .flat_map(|reg| reg.splitx(other.minx))
                        .flatten()
                        .flat_map(|reg| reg.splitx(other.maxx + 1))
                        .flatten()
                        .flat_map(|reg| reg.splity(other.miny))
                        .flatten()
                        .flat_map(|reg| reg.splity(other.maxy + 1))
                        .flatten()
                        .flat_map(|reg| reg.splitz(other.minz))
                        .flatten()
                        .flat_map(|reg| reg.splitz(other.maxz + 1))
                        .flatten()
                        .filter(|reg| !other.contains(reg))
                        .collect();
                    splitregs
                }
            }
        }
    }
}

fn on(region: &Region, on_regions: HashSet<Region>) -> HashSet<Region> {
    if on_regions.is_empty() {
        HashSet::from([*region])
    } else {
        on_regions
            .iter()
            .flat_map(|reg| reg.merge_around(region))
            .chain([*region])
            .collect()
    }
}

fn off(region: &Region, on_regions: HashSet<Region>) -> HashSet<Region> {
    on_regions
        .iter()
        .flat_map(|reg| reg.merge_around(region))
        .collect()
}

enum Instruction {
    On(Region),
    Off(Region),
}
impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let (instruct, region) = s.split_once(" ").unwrap();
        match instruct {
            "on" => Ok(Instruction::On(Region::from_str(region).unwrap())),
            "off" => Ok(Instruction::Off(Region::from_str(region).unwrap())),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

fn process_instructions(input: &[String]) -> HashSet<Region> {
    parse_input(input)
        .iter()
        .fold(
            HashSet::new(),
            |on_regions, instruction| match instruction {
                Instruction::On(region) => on(region, on_regions),
                Instruction::Off(region) => off(region, on_regions),
            },
        )
}

fn p1(regions: &HashSet<Region>) -> i64 {
    let core_region = Region {
        minx: -50,
        maxx: 50,
        miny: -50,
        maxy: 50,
        minz: -50,
        maxz: 50,
    };
    regions
        .iter()
        .filter_map(|reg| core_region.overlap(reg))
        .map(|reg| reg.size())
        .sum::<i64>()
}
fn p2(regions: &HashSet<Region>) -> i64 {
    regions.iter().map(|reg| reg.size()).sum::<i64>()
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 22 ===");

    let (regions, time_regions) = run_and_print_time(process_instructions, &input);
    println!("Processed instructions.");

    let (a, timea) = run_and_print_time(p1, &regions);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &regions);
    println!("Part2: {}", b);

    time_regions + timea + timeb
}
