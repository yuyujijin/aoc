mod days;
use std::time::Instant;

use days::{day01, day02, day03, day04};

use clap::Parser;

/// Request a day to run
#[derive(Parser)]
struct Cli {
    /// The requested day to run
    days: Vec<f32>,
}

fn get_day_solver(day: i32) -> fn(i32) -> i32 {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Cli = Cli::parse();

    for day in args.days {
        println!("# Solving day #{}...", day);

        // Retrieve day and part
        let [day, part] = [day.trunc() as i32, (day.fract() * 10.0).round() as i32];

        if part < 0 || part > 2 {
            panic!(" * Incorrect part value ({}). Must be 1 or 2.", part)
        }

        // Retrieve the request day
        let problem: fn(i32) -> i32 = get_day_solver(day);

        // Retrieve time
        let start: Instant = Instant::now();

        // Compute the solution
        let solution: i32 = problem(part);

        println!(" * Solution found: {}", solution);

        println!(
            " * Problem solved in {:.4?}s",
            start.elapsed().as_secs_f64()
        )
    }
}
