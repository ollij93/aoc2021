use std::time::Instant;

/// Run the given function and print how long it took to run in milliseconds
pub fn run_and_print_time<T, F, I>(f: F, i: I) -> (T, u128)
where
    F: Fn(I) -> T,
{
    let now = Instant::now();
    let ret = f(i);
    let time = now.elapsed().as_micros();
    println!("Ran in {}µs.", time);
    (ret, time)
}

/// Calculate the median of a set of values
pub fn median<I>(values: &[I]) -> I
where
    I: std::cmp::Ord + std::clone::Clone + Copy,
{
    let mut copy: Vec<I> = values.to_owned();
    copy.sort_unstable();
    copy[copy.len() / 2]
}

/// Calculate the mean of a set of values
pub fn mean(values: &[u32]) -> u32 {
    values.iter().sum::<u32>() / values.len() as u32
}
