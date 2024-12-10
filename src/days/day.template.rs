use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/dayn.txt";

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(path: &str) -> i64 {
    if let Ok(lines) = read_lines(path) {
        // Do something...
    } else {
        return -1;
    }
    return -1;
}

fn part2(path: &str) -> i64 {
    if let Ok(lines) = read_lines(path) {
        // Do something...
    } else {
        return -1;
    }
    return -1;
}

pub fn solve(part: i64) -> i64 {
    match part {
        1 => part1(INPUT_FILE_PATH),
        2 => part2(INPUT_FILE_PATH),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/dayn.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i64 = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = -1;
        assert_eq!(result, expected);
    }
}
