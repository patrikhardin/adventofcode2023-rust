mod day01;
mod day02;

use std::{fmt::Display, time::Instant};

fn main() {
    let start = Instant::now();

    time(1, 1, day01::part1);
    time(1, 2, day01::part2);
    time(2, 1, day02::part1);
    time(2, 2, day02::part2);

    let end = Instant::now();
    println!("Total Elapsed: {:?}", end.duration_since(start));
}

fn time<T, F>(day: u8, part: u8, f: F)
where
    T: Display,
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let end = Instant::now();

    println!(
        "Day {:02} Part {}: | Answer {:<5} | Elapsed: {:?}",
        day,
        part,
        result,
        end.duration_since(start)
    );
}
