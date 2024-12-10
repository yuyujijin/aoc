use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day05.txt";

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
    let mut count: i64 = 0;
    let mut map: HashMap<i64, Vec<i64>> = HashMap::<i64, Vec<i64>>::new();
    if let Ok(lines) = read_lines(path) {
        let mut index: usize = 0;
        for line in lines.flatten() {
            match index {
                // First block
                0 => {
                    // Reached next section
                    if line.len() == 0 {
                        index += 1;
                        continue;
                    }
                    let numbers: Vec<i64> =
                        line.split('|').map(|f| f.parse::<i64>().unwrap()).collect();
                    let (left, right): (i64, i64) =
                        (*numbers.get(0).unwrap(), *numbers.get(1).unwrap());
                    // Greater than
                    map.entry(left).or_insert(Vec::new()).push(right);
                }
                // Second block
                1 => {
                    let numbers: Vec<i64> =
                        line.split(',').map(|f| f.parse::<i64>().unwrap()).collect();
                    let mut copy: Vec<i64> = numbers.clone();
                    sort_updates(&map, &mut copy);
                    if vec_equals(&numbers, &copy) {
                        count += numbers.get(numbers.len() / 2).unwrap();
                    }
                }
                _ => (),
            }
        }
        return count;
    }
    return -1;
}

fn part2(path: &str) -> i64 {
    let mut count: i64 = 0;
    let mut map: HashMap<i64, Vec<i64>> = HashMap::<i64, Vec<i64>>::new();
    if let Ok(lines) = read_lines(path) {
        let mut index: usize = 0;
        for line in lines.flatten() {
            match index {
                // First block
                0 => {
                    // Reached next section
                    if line.len() == 0 {
                        index += 1;
                        continue;
                    }
                    let numbers: Vec<i64> =
                        line.split('|').map(|f| f.parse::<i64>().unwrap()).collect();
                    let (left, right): (i64, i64) =
                        (*numbers.get(0).unwrap(), *numbers.get(1).unwrap());
                    // Greater than
                    map.entry(left).or_insert(Vec::new()).push(right);
                }
                // Second block
                1 => {
                    let numbers: Vec<i64> =
                        line.split(',').map(|f| f.parse::<i64>().unwrap()).collect();
                    let mut copy: Vec<i64> = numbers.clone();
                    sort_updates(&map, &mut copy);
                    if !vec_equals(&numbers, &copy) {
                        count += copy.get(numbers.len() / 2).unwrap();
                    }
                }
                _ => (),
            }
        }
        return count;
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

fn compare(map: &HashMap<i64, Vec<i64>>, a: &i64, b: &i64) -> std::cmp::Ordering {
    match map.get(a) {
        Some(l) => {
            if !l.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }
        _ => return std::cmp::Ordering::Greater,
    }
    return std::cmp::Ordering::Less;
}

fn sort_updates(map: &HashMap<i64, Vec<i64>>, numbers: &mut Vec<i64>) {
    numbers.sort_by(|a, b| compare(map, a, b));
}

fn vec_equals(left: &Vec<i64>, right: &Vec<i64>) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for i in 0..left.len() {
        let (a, b): (&i64, &i64) = (left.get(i).unwrap(), right.get(i).unwrap());
        if a != b {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day05.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i64 = 143;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = 123;
        assert_eq!(result, expected);
    }
}
