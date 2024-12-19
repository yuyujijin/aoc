mod days;
mod utils;
use std::time::Instant;

use clap::Parser;
use days::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10};

/// Request a day to run
#[derive(Parser)]
struct Cli {
    /// The requested day to run
    days: Vec<f32>,
}

fn get_day_solver(day: i64) -> fn(i64) -> i64 {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Cli = Cli::parse();

    for day in args.days {
        println!("# Solving day #{}...", day);

        // Retrieve day and part
        let [day, part] = [day.trunc() as i64, (day.fract() * 10.0).round() as i64];

        if part < 0 || part > 2 {
            panic!(" * Incorrect part value ({}). Must be 1 or 2.", part)
        }

        // Retrieve the request day
        let problem: fn(i64) -> i64 = get_day_solver(day);

        // Retrieve time
        let start: Instant = Instant::now();

        // Compute the solution
        let solution: i64 = problem(part);

        println!(" * Solution found: {}", solution);

        println!(
            " * Problem solved in {:.4?}s",
            start.elapsed().as_secs_f64()
        )
    }
}
