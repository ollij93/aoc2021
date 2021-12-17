// Solutions for day17 of Advent of Code

use std::ops::RangeInclusive;

use super::common::run_and_print_time;

fn parse_input(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (_, coords) = input.split_once(": ").unwrap();
    let (xeq, yeq) = coords.split_once(", ").unwrap();

    let (_, xr) = xeq.split_once("=").unwrap();
    let (xstart, xend) = xr.split_once("..").unwrap();

    let (_, yr) = yeq.split_once("=").unwrap();
    let (ystart, yend) = yr.split_once("..").unwrap();

    (
        xstart.parse::<i32>().unwrap()..=xend.parse::<i32>().unwrap(),
        ystart.parse::<i32>().unwrap()..=yend.parse::<i32>().unwrap(),
    )
}

/// Calculate the velocity needed to make the pod stop as soon after the given
/// positiion as possible
fn min_x_velocity(xr: &RangeInclusive<i32>) -> i32 {
    let xtgt = *xr.start();
    let mut xvel = 1;
    let mut xpos = 1;
    while xpos < xtgt {
        xvel += 1;
        xpos += xvel;
    }
    xvel
}

fn max_y_velocity(yr: &RangeInclusive<i32>) -> i32 {
    // When falling with y we always return back to y = 0 with negative our
    // initial velocity
    // So if we're falling below this point, the fastest we can be going at
    // that point is such that we end up right at the end of range.
    // Otherwise, its such that the step before we're right at the top of the
    // range
    if *yr.start() < 0 {
        -yr.start() - 1
    } else {
        *yr.end()
    }
}

fn enters_ranges(
    mut xvel: i32,
    mut yvel: i32,
    xr: &RangeInclusive<i32>,
    yr: &RangeInclusive<i32>,
) -> bool {
    let mut xpos = 0;
    let mut ypos = 0;
    // Assuming positive X and negative y ranges
    while xpos <= *xr.end() && ypos >= *yr.start() {
        xpos += xvel;
        if xvel > 0 {
            xvel -= 1;
        }
        ypos += yvel;
        yvel -= 1;

        if xr.contains(&xpos) && yr.contains(&ypos) {
            return true;
        }
    }
    false
}

fn p1(input: &[String]) -> i32 {
    let (_, yr) = parse_input(&input[0]);

    let yvel = max_y_velocity(&yr);

    (0..=yvel).sum()
}

fn p2(input: &[String]) -> u32 {
    // Bounds of the space to search are:
    //   minX => stops as soon getting within range
    //   maxX => in a single step hits the back of the range
    //   maxY => calculated in part 1
    //   minY => in a single step hits the bottom of the range (if negative)
    //           stops as soon as getting within range (if positive)
    //               - ignore because not relavant to our example!
    let (xr, yr) = parse_input(&input[0]);
    // These are calculated assuming we're going into positive x space
    // - fine for our examples
    let xmin = min_x_velocity(&xr);
    let xmax = xr.end();
    let ymax = max_y_velocity(&yr);
    let ymin = yr.start();

    let mut count = 0;
    for xvel in xmin..=*xmax {
        for yvel in *ymin..=ymax {
            if enters_ranges(xvel, yvel, &xr, &yr) {
                count += 1;
            }
        }
    }
    count
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 17 ===");

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
    fn test_part1() {
        assert_eq!(p1(&["target area: x=20..30, y=-10..-5".to_string()]), 45);
    }
    #[test]
    fn test_part2() {
        assert_eq!(p2(&["target area: x=20..30, y=-10..-5".to_string()]), 112);
    }
}
