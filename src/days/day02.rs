use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Skip;
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day02.txt";

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
    let mut counter: i32 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // Parse the line
            let splitted: Vec<i32> = line
                .split_whitespace()
                .map(|f| f.parse::<i32>().unwrap())
                .collect();
            // Iterate through the lines
            if is_safe(&splitted) {
                counter += 1;
            }
        }
    } else {
        return -1;
    }
    return counter;
}

fn part2(path: &str) -> i32 {
    let mut counter: i32 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // Parse the line
            let splitted: Vec<i32> = line
                .split_whitespace()
                .map(|f| f.parse::<i32>().unwrap())
                .collect();
            // * Check if safe dampenered
            match is_safe_dampenered(&splitted)
            // * If not, check if without first element is safe
                || is_safe(&splitted[1..].to_vec())
            // * If not, check without second element is safe
                || is_safe(&[&splitted[0..1], &splitted[2..]].concat().to_vec())
            {
                true => counter += 1,
                _ => (),
            }
        }
    } else {
        return -1;
    }
    return counter;
}

pub fn solve(part: i32) -> i32 {
    match part {
        1 => part1(INPUT_FILE_PATH),
        2 => part2(INPUT_FILE_PATH),
        _ => unimplemented!(),
    }
}

fn is_safe(elems: &Vec<i32>) -> bool {
    let n: usize = elems.len();
    static DELTA_MAX: i32 = 3;

    if n < 2 {
        return true;
    }

    let [first, second] = [elems.get(0).unwrap(), elems.get(1).unwrap()];

    let compare: fn(i32, i32) -> bool = if first < second {
        |a: i32, b: i32| a < b && (a - b).abs() <= DELTA_MAX
    } else {
        |a: i32, b: i32| a > b && (a - b).abs() <= DELTA_MAX
    };

    for i in 0..n - 1 {
        let [n, m]: [i32; 2] = [*elems.get(i).unwrap(), *elems.get(i + 1).unwrap()];
        if !compare(n, m) {
            return false;
        }
    }
    return true;
}

fn is_safe_dampenered(elems: &Vec<i32>) -> bool {
    let size: usize = elems.len();
    static DELTA_MAX: i32 = 3;

    if size < 2 {
        return true;
    }

    let [first, second] = [elems.get(0).unwrap(), elems.get(1).unwrap()];

    let compare: fn(i32, i32) -> bool = if first < second {
        |a: i32, b: i32| a < b && (a - b).abs() <= DELTA_MAX
    } else {
        |a: i32, b: i32| a > b && (a - b).abs() <= DELTA_MAX
    };

    let mut dampenered: bool = false;

    let mut i: usize = 0;
    while i < size - 1 {
        let [n, m]: [i32; 2] = [*elems.get(i).unwrap(), *elems.get(i + 1).unwrap()];
        if !compare(n, m) {
            // Already dampenered once -> Can't do it again sorry :(
            if dampenered {
                return false;
            }
            // Check if it is safe to dampener
            if !is_safe_to_dampener(&elems[i..], compare) {
                return false;
            }
            // Skip next element (since it's skipped)
            i += 1;
            dampenered = true;
        }
        i += 1;
    }
    return true;
}

fn is_safe_to_dampener(elems: &[i32], compare: fn(i32, i32) -> bool) -> bool {
    let size: usize = elems.len();
    return match size {
        0 | 1 | 2 => true,
        _ => compare(*elems.get(0).unwrap(), *elems.get(2).unwrap()),
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day02.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i32 = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i32 = 4;
        assert_eq!(result, expected);
    }
}
