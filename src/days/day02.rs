use std::fs::File;
use std::io::{self, BufRead};
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

fn vec_without(vec: &Vec<i64>, idx: usize) -> Vec<i64> {
    if idx > vec.len() {
        return Vec::<i64>::new();
    }
    return [&vec[0..idx], &vec[idx + 1..]].concat().to_vec();
}

fn part1(path: &str) -> i64 {
    let mut counter: i64 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // Parse the line
            let splitted: Vec<i64> = line
                .split_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect();
            // Iterate through the lines
            if is_safe(&splitted, None) {
                counter += 1;
            }
        }
    } else {
        return -1;
    }
    return counter;
}

fn part2(path: &str) -> i64 {
    let mut counter: i64 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // Parse the line
            let splitted: Vec<i64> = line
                .split_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect();
            // * Check if safe dampenered
            match is_safe_dampenered(&splitted)
            // * If not, check if without first element is safe
            || is_safe(&vec_without(&splitted, 0), None)
            // * If not, check without second element is safe
            || is_safe(&vec_without(&splitted, 1), None)
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

pub fn solve(part: i64) -> i64 {
    match part {
        1 => part1(INPUT_FILE_PATH),
        2 => part2(INPUT_FILE_PATH),
        _ => unimplemented!(),
    }
}

fn is_safe(elems: &Vec<i64>, compare: Option<fn(i64, i64) -> bool>) -> bool {
    let n: usize = elems.len();
    static DELTA_MAX: i64 = 3;

    if n < 2 {
        return true;
    }

    let [first, second] = [elems.get(0).unwrap(), elems.get(1).unwrap()];

    let compare: fn(i64, i64) -> bool = match compare {
        Some(f) => f,
        _ => {
            if first < second {
                |a: i64, b: i64| a < b && (a - b).abs() <= DELTA_MAX
            } else {
                |a: i64, b: i64| a > b && (a - b).abs() <= DELTA_MAX
            }
        }
    };

    if !compare(*first, *second) {
        return false;
    }

    return is_safe(&elems[1..].to_vec(), Some(compare));
}

fn is_safe_dampenered(elems: &Vec<i64>) -> bool {
    let size: usize = elems.len();
    static DELTA_MAX: i64 = 3;

    if size < 2 {
        return true;
    }

    let [first, second] = [elems.get(0).unwrap(), elems.get(1).unwrap()];

    let compare: fn(i64, i64) -> bool = if first < second {
        |a: i64, b: i64| a < b && (a - b).abs() <= DELTA_MAX
    } else {
        |a: i64, b: i64| a > b && (a - b).abs() <= DELTA_MAX
    };

    let mut i: usize = 0;
    while i < size - 1 {
        let [n, m]: [i64; 2] = [*elems.get(i).unwrap(), *elems.get(i + 1).unwrap()];
        if !compare(n, m) {
            return is_safe(&vec_without(elems, i), None)
                || is_safe(&vec_without(elems, i + 1), None);
        }
        i += 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day02.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i64 = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = 4;
        assert_eq!(result, expected);
    }
}
