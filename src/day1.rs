// Solutions for day1 of Advent of Code

pub fn p1(input: Vec<u32>) -> u32 {
    let mut ret: u32 = 0;

    for (a, b) in input.iter().zip(&input[1..]) {
        if a < b { ret += 1; }
    }

    return ret;
}

pub fn p2(input: Vec<u32>) -> u32 {
    // Calculate the 3 element sums and then re-run part1 on those values.
    let mut sums: Vec<u32> = Vec::new();
    for ((a, b), c) in input.iter().zip(&input[1..]).zip(&input[2..]) {
        sums.push(a + b + c);
    }

    return p1(sums);
}
