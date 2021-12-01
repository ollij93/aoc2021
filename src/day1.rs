use std::io;
use std::io::BufRead;

pub fn p1() {
    let mut prev_value: Option<i32> = None;
    let mut count: i32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let value: i32 = line.unwrap().parse::<i32>().unwrap();
        if prev_value.is_some() && prev_value.unwrap() < value {
            count = count + 1;
        }
        prev_value = Some(value);
    }

    println!("{}", count);
}

pub fn p2() {
    let mut v0: Option<i32> = None; // Sum of previous three numbers
    let mut v1: Option<i32> = None; // Sum of previous two numbers
    let mut v2: Option<i32> = None; // Previous number
    let mut count: i32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let value: i32 = line.unwrap().parse::<i32>().unwrap();

        // Calculate the new values for the numbers
        let mut new_v0: Option<i32> = None;
        let mut new_v1: Option<i32> = None;

        if v2.is_some() {
            new_v1 = Some(v2.unwrap() + value);
        }

        if v1.is_some() {
            new_v0 = Some(v1.unwrap() + value);
        }

        if v0.is_some() {
            // Compare the old and new values of the blocks of 3
            if v0.unwrap() < new_v0.unwrap() {
                count = count + 1;
            }
        }

        v0 = new_v0;
        v1 = new_v1;
        v2 = Some(value);
    }

    println!("{}", count);
}