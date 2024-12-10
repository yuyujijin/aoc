use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day01.txt";

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
    let mut distances: i64 = 0;
    if let Ok(lines) = read_lines(path) {
        let mut left_list: Vec<i64> = Vec::new();
        let mut right_list: Vec<i64> = Vec::new();
        // Retrieve columns
        for line in lines.flatten() {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            let [left, right]: [i64; 2] = [
                splitted.get(0).unwrap().parse::<i64>().unwrap(),
                splitted.get(1).unwrap().parse::<i64>().unwrap(),
            ];
            left_list.push(left);
            right_list.push(right);
        }
        // Sort both lists
        left_list.sort();
        right_list.sort();
        // Iterate through, calculting distances
        for (a, b) in zip(left_list, right_list) {
            distances += (a - b).abs();
        }
    } else {
        return -1;
    }
    return distances;
}

fn part2(path: &str) -> i64 {
    let mut distances: i64 = 0;
    if let Ok(lines) = read_lines(path) {
        let mut left_list: Vec<i64> = Vec::new();
        let mut right_map: HashMap<i64, i64> = HashMap::new();
        // Retrieve columns
        for line in lines.flatten() {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            let [left, right]: [i64; 2] = [
                splitted.get(0).unwrap().parse::<i64>().unwrap(),
                splitted.get(1).unwrap().parse::<i64>().unwrap(),
            ];
            left_list.push(left);
            // Increase map[key]
            increase_hash_map(&mut right_map, right);
        }
        // Iterate through left and multiply by occurences in right if exists in right
        for key in left_list {
            if let Some(occurences) = right_map.get(&key) {
                distances += key * occurences;
            }
        }
    } else {
        return -1;
    }
    return distances;
}

fn increase_hash_map(map: &mut HashMap<i64, i64>, key: i64) {
    let count = map.entry(key).or_insert(0);
    *count += 1;
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

    static EXAMPLE_FILE_PATH: &str = "inputs/day01.example.txt";

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
