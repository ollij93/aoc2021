use std::time::Instant;

/// Run the given function and print how long it took to run in milliseconds
pub fn run_and_print_time<T, F, I>(f: F, i: I) -> T where F: Fn(I) -> T {
    let now = Instant::now();
    let ret = f(i);
    println!("Ran in {}µs.", now.elapsed().as_micros());
    return ret;
}
