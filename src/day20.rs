// Solutions for day20 of Advent of Code

use super::common::run_and_print_time;
use crate::point::PointI;

fn parse_input(input: &[String]) -> (Vec<bool>, Vec<Vec<bool>>) {
    let algo = input[0].chars().map(|c| c == '#').collect::<Vec<bool>>();
    let image = input[2..]
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    (algo, image)
}

fn vec_for_x(x: i32, imagerow: &[bool], infinity_val: bool) -> Vec<bool> {
    let rowlen = imagerow.len() as i32;
    match x {
        x if x < -1 || x > rowlen => vec![infinity_val, infinity_val, infinity_val],
        x if x == -1 => vec![infinity_val, infinity_val, imagerow[0]],
        x if x == 0 => vec![infinity_val, imagerow[0], imagerow[1]],
        x if x == rowlen - 1 => vec![imagerow[x as usize - 1], imagerow[x as usize], infinity_val],
        x if x == rowlen => vec![imagerow[x as usize - 1], infinity_val, infinity_val],
        x => vec![
            imagerow[x as usize - 1],
            imagerow[x as usize],
            imagerow[x as usize + 1],
        ],
    }
}

fn vec_for_point(point: &PointI, image: &[Vec<bool>], infinity_val: bool) -> Vec<bool> {
    let imagelen = image.len() as i32;
    let vecsvec: Vec<Vec<bool>> = match point.y {
        y if y < -1 || y > imagelen => vec![
            vec![infinity_val, infinity_val, infinity_val],
            vec![infinity_val, infinity_val, infinity_val],
            vec![infinity_val, infinity_val, infinity_val],
        ],
        y if y == -1 => vec![
            vec![infinity_val, infinity_val, infinity_val],
            vec![infinity_val, infinity_val, infinity_val],
            vec_for_x(point.x, &image[0], infinity_val),
        ],
        y if y == 0 => {
            vec![
                vec![infinity_val, infinity_val, infinity_val],
                vec_for_x(point.x, &image[0], infinity_val),
                vec_for_x(point.x, &image[1], infinity_val),
            ]
        }
        y if y == imagelen - 1 => vec![
            vec_for_x(point.x, &image[y as usize - 1], infinity_val),
            vec_for_x(point.x, &image[y as usize], infinity_val),
            vec![infinity_val, infinity_val, infinity_val],
        ],
        y if y == imagelen => vec![
            vec_for_x(point.x, &image[y as usize - 1], infinity_val),
            vec![infinity_val, infinity_val, infinity_val],
            vec![infinity_val, infinity_val, infinity_val],
        ],
        y => vec![
            vec_for_x(point.x, &image[y as usize - 1], infinity_val),
            vec_for_x(point.x, &image[y as usize], infinity_val),
            vec_for_x(point.x, &image[y as usize + 1], infinity_val),
        ],
    };
    vecsvec.iter().flatten().copied().collect::<Vec<bool>>()
}

fn val_for_point(point: &PointI, image: &[Vec<bool>], infinity_val: bool) -> usize {
    let vec = vec_for_point(point, image, infinity_val);
    vec.iter()
        .fold(0, |val, bit| if *bit { val * 2 + 1 } else { val * 2 })
}

fn enhance(image: &[Vec<bool>], algo: &[bool], infinity_val: bool) -> (Vec<Vec<bool>>, bool) {
    let newimage = (-1..=image.len() as i32).fold(vec![], |rows, ri| {
        rows.iter()
            .chain(
                // Assuming image is square
                &vec![(-1..=image[0].len() as i32).fold(vec![], |row, ci| {
                    row.iter()
                        .chain(&vec![
                            algo[val_for_point(&PointI { x: ci, y: ri }, image, infinity_val)],
                        ])
                        .copied()
                        .collect::<Vec<bool>>()
                })],
            )
            .cloned()
            .collect::<Vec<Vec<bool>>>()
    });
    let newinfinity = algo[val_for_point(&PointI { x: -2, y: -2 }, image, infinity_val)];
    (newimage, newinfinity)
}

fn _display(image: &[Vec<bool>], padding: usize, inf: bool) {
    let width = image[0].len() - 1;
    let pad = if inf { "#" } else { "." };
    // Top padding
    for _ in 0..padding {
        for _ in 0..(width + 2 * padding) {
            print!("{}", pad);
        }
        println!("{}", pad);
    }
    for row in image {
        // Left padding
        for _ in 0..padding {
            print!("{}", pad);
        }
        // Actual image
        print!(
            "{}",
            row.iter()
                .map(|bit| if *bit { "#" } else { "." })
                .collect::<Vec<&str>>()
                .join("")
        );
        // Right padding
        for _ in 0..padding {
            print!("{}", pad);
        }
        println!();
    }
    // Bottom padding
    for _ in 0..padding {
        for _ in 0..(width + 2 * padding) {
            print!("{}", pad);
        }
        println!("{}", pad);
    }
}

fn p1(input: &[String]) -> usize {
    let (algo, image) = parse_input(input);
    let (final_image, _final_inf) = (0..2).fold((image, false), |(img, inf), _| {
        //_display(&img, 3, inf);
        enhance(&img, &algo, inf)
    });
    //_display(&final_image, 3, _final_inf);
    final_image.iter().flatten().filter(|b| **b).count()
}

fn p2(input: &[String]) -> usize {
    let (algo, image) = parse_input(input);
    let (final_image, _final_inf) = (0..50).fold((image, false), |(img, inf), _| {
        //_display(&img, 3, inf);
        enhance(&img, &algo, inf)
    });
    //_display(&final_image, 3, _final_inf);
    final_image.iter().flatten().filter(|b| **b).count()
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 20 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {}", b);

    timea + timeb
}
