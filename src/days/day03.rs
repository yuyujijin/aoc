use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day03.txt";

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(path: &str) -> i32 {
    if let Ok(lines) = read_lines(path) {
        return lines
            .flatten()
            // Compute for each line
            .map(|line| compute(&line))
            .reduce(|x, acc| x + acc)
            .unwrap();
    }
    return -1;
}

fn part2(path: &str) -> i32 {
    if let Ok(lines) = read_lines(path) {
        let full_line = lines
            .flatten()
            // Create a single string concatening each line
            .reduce(|x, acc| x + &acc)
            .unwrap();
        return compute_do(&full_line);
    }
    return -1;
}

pub fn solve(part: i32) -> i32 {
    match part {
        1 => part1(INPUT_FILE_PATH),
        2 => part2(INPUT_FILE_PATH),
        _ => unimplemented!(),
    }
}

fn compute(instructions: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re
        // Iterate throuh captures
        .captures_iter(&instructions)
        // Map on first and second group multiplication (as i32)
        .map(|m| {
            let (_, [left, right]) = m.extract();
            left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
        })
        // Reduce the array by adding every elts
        .reduce(|x, acc| x + acc)
        .unwrap()
}

fn compute_do(instruction: &str) -> i32 {
    // Catch smallest `(begging OR "do()") anything (don't() OR end)` group
    // Those represents valid groups
    let re: Regex = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    re
        // Iterate throuh captures
        .captures_iter(&instruction)
        // Map on first and second group multiplication (as i32)
        .map(|m| {
            let (_, [instructions]) = m.extract();
            compute(&instructions)
        })
        // Reduce the array by adding every elts
        .reduce(|x, acc| x + acc)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day03.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i32 = 161;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2("inputs/day03.02.example.txt");
        let expected: i32 = 48;
        assert_eq!(result, expected);
    }
}
