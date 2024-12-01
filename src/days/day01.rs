use std::collections::HashMap;
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
    let mut distances: i32 = 0;
    if let Ok(lines) = read_lines("inputs/day01.txt") {
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();
        // Retrieve columns
        for line in lines.flatten() {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            let [left, right]: [i32; 2] = [
                splitted.get(0).unwrap().parse::<i32>().unwrap(),
                splitted.get(1).unwrap().parse::<i32>().unwrap(),
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

fn part2() -> i32 {
    let mut distances: i32 = 0;
    if let Ok(lines) = read_lines("inputs/day01.txt") {
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_map: HashMap<i32, i32> = HashMap::new();
        // Retrieve columns
        for line in lines.flatten() {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            let [left, right]: [i32; 2] = [
                splitted.get(0).unwrap().parse::<i32>().unwrap(),
                splitted.get(1).unwrap().parse::<i32>().unwrap(),
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

fn increase_hash_map(map: &mut HashMap<i32, i32>, key: i32) {
    let count = map.entry(key).or_insert(0);
    *count += 1;
}

pub fn solve(part: i32) -> i32 {
    match part {
        1 => part1(),
        2 => part2(),
        _ => unimplemented!(),
    }
}
