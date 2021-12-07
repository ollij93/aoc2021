use std::time::Instant;

/// Run the given function and print how long it took to run in milliseconds
pub fn run_and_print_time<T, F, I>(f: F, i: I) -> T
where
    F: Fn(I) -> T,
{
    let now = Instant::now();
    let ret = f(i);
    println!("Ran in {}µs.", now.elapsed().as_micros());
    ret
}

/// Calculate the median of a set of values
pub fn median(values: &[u32]) -> u32 {
    let mut copy: Vec<u32> = values.to_owned();
    copy.sort_unstable();
    copy[copy.len() / 2]
}

/// Calculate the mean of a set of values
pub fn mean(values: &[u32]) -> u32 {
    values.iter().sum::<u32>() / values.len() as u32
}
