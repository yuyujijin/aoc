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

fn part1(path: &str) -> i32 {
    let mut count: i32 = 0;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::<i32, Vec<i32>>::new();
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
                    let numbers: Vec<i32> =
                        line.split('|').map(|f| f.parse::<i32>().unwrap()).collect();
                    let (left, right): (i32, i32) =
                        (*numbers.get(0).unwrap(), *numbers.get(1).unwrap());
                    // Greater than
                    map.entry(left).or_insert(Vec::new()).push(right);
                }
                // Second block
                1 => {
                    let numbers: Vec<i32> =
                        line.split(',').map(|f| f.parse::<i32>().unwrap()).collect();
                    let mut copy: Vec<i32> = numbers.clone();
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

fn part2(path: &str) -> i32 {
    let mut count: i32 = 0;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::<i32, Vec<i32>>::new();
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
                    let numbers: Vec<i32> =
                        line.split('|').map(|f| f.parse::<i32>().unwrap()).collect();
                    let (left, right): (i32, i32) =
                        (*numbers.get(0).unwrap(), *numbers.get(1).unwrap());
                    // Greater than
                    map.entry(left).or_insert(Vec::new()).push(right);
                }
                // Second block
                1 => {
                    let numbers: Vec<i32> =
                        line.split(',').map(|f| f.parse::<i32>().unwrap()).collect();
                    let mut copy: Vec<i32> = numbers.clone();
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

pub fn solve(part: i32) -> i32 {
    match part {
        1 => part1(INPUT_FILE_PATH),
        2 => part2(INPUT_FILE_PATH),
        _ => unimplemented!(),
    }
}

fn compare(map: &HashMap<i32, Vec<i32>>, a: &i32, b: &i32) -> std::cmp::Ordering {
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

fn sort_updates(map: &HashMap<i32, Vec<i32>>, numbers: &mut Vec<i32>) {
    numbers.sort_by(|a, b| compare(map, a, b));
}

fn vec_equals(left: &Vec<i32>, right: &Vec<i32>) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for i in 0..left.len() {
        let (a, b): (&i32, &i32) = (left.get(i).unwrap(), right.get(i).unwrap());
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
        let expected: i32 = 143;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i32 = 123;
        assert_eq!(result, expected);
    }
}
