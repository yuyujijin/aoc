use std::fs::File;
use std::io::{self, BufRead};
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

fn part1() -> String {
    let lines = read_lines("inputs/dayn-1.txt").unwrap();
    for line in lines {
        // Do something...
    }
    return String::from("part1");
}

fn part2() -> String {
    let lines = read_lines("inputs/dayn-1.txt").unwrap();
    for line in lines {
        // Do something...
    }
    return String::from("part1");
}

pub fn solve(part: i32) -> String {
    match part {
        1 => part1(),
        2 => part2(),
        _ => unimplemented!(),
    }
}
