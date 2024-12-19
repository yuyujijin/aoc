use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::utils::runner::parallelize;

static INPUT_FILE_PATH: &str = "inputs/day07.txt";

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
    let add = |a, b| a + b;
    let mul = |a, b| a * b;
    if let Ok(lines) = read_lines(path) {
        return parallelize(
            lines
                .flatten()
                .map(|line| return move || compute(&line, &[add, mul].to_vec()))
                .collect(),
        );
    }
    return -1;
}

fn part2(path: &str) -> i64 {
    let add = |a, b| a + b;
    let mul = |a, b| a * b;
    let or = |a, b| format!("{}{}", a, b).parse::<i64>().unwrap();
    if let Ok(lines) = read_lines(path) {
        return parallelize(
            lines
                .flatten()
                .map(|line| return move || compute(&line, &[add, mul, or].to_vec()))
                .collect(),
        );
    }
    return -1;
}

fn compute(str: &String, operations: &Vec<fn(i64, i64) -> i64>) -> i64 {
    let splitted: Vec<&str> = str.split(':').collect();
    let (total, numbers): (i64, Vec<i64>) = (
        splitted.get(0).unwrap().parse::<i64>().unwrap(),
        splitted
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    );
    return if combinaisons(&numbers, &operations, total)
        .iter()
        .any(|x| *x == total)
    {
        total
    } else {
        0
    };
}

fn combinaisons(
    numbers: &[i64],
    operations: &Vec<fn(i64, i64) -> i64>,
    max_value: i64,
) -> Vec<i64> {
    match numbers.len() {
        0 => return [].to_vec(),
        1 => return [*numbers.get(0).unwrap()].to_vec(),
        _ => {}
    }

    let (first, second) = (*numbers.get(0).unwrap(), *numbers.get(1).unwrap());
    // Create combinaisons for every operations
    let computeds: Vec<Vec<i64>> = operations
        .iter()
        // Apply operations
        .map(|f| [[f(first, second)].to_vec(), numbers[2..].to_vec()].concat())
        // Remove numbers that exceed `max_value`` (no substraction operation provided)
        .filter(|l| l.get(0).unwrap() <= &max_value)
        .collect();
    // Compute childrens
    return computeds
        .iter()
        .map(|n| combinaisons(&n, &operations, max_value))
        .collect::<Vec<Vec<i64>>>()
        .concat();
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

    static EXAMPLE_FILE_PATH: &str = "inputs/day07.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i64 = 3749;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = 11387;
        assert_eq!(result, expected);
    }
}
