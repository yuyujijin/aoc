mod days;
use std::time::Instant;

use days::day01;

use clap::Parser;

/// Request a day to run
#[derive(Parser)]
struct Cli {
    /// The requested day to run
    day: f32,
}

fn get_day_solver(day: i32) -> fn(i32) -> String {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Cli = Cli::parse();

    println!("Solving day #{}...", args.day);

    // Retrieve day and part
    let [day, part] = [
        args.day.trunc() as i32,
        (args.day.fract() * 10.0).round() as i32,
    ];

    if part < 0 || part > 2 {
        panic!("Incorrect part value ({}). Must be 1 or 2.", part)
    }

    // Retrieve the request day
    let problem: fn(i32) -> String = get_day_solver(day);

    // Compute the solution
    let solution: String = problem(part);

    // Retrieve time
    let end: Instant = Instant::now();

    let elapsed_ms: f64 = (end).elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("Solution found: {}", solution);

    println!("Problem solved in {}ms", elapsed_ms)
}
