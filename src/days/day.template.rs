use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() -> i32 {
    if let Ok(lines) = read_lines("inputs/dayn-1.txt") {
        // Do something...
    } else {
        return -1;
    }
    return distances;
}

fn part2() -> i32 {
    let mut distances: i32 = 0;
    if let Ok(lines) = read_lines("inputs/dayn-2.txt") {
        // Do something...
    } else {
        return -1;
    }
    return -1;
}

pub fn solve(part: i32) -> i32 {
    match part {
        1 => part1(),
        2 => part2(),
        _ => unimplemented!(),
    }
}
