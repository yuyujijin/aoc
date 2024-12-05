use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

static INPUT_FILE_PATH: &str = "inputs/day04.txt";

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
    let mut matrix: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // Retrieve the input as full line
            matrix.push(line.chars().collect());
            // Construct a string for the diagonals
        }
        return compute_1(&matrix).try_into().unwrap();
    }
    return -1;
}

fn part2(path: &str) -> i32 {
    let mut matrix: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // Retrieve the input as full line
            matrix.push(line.chars().collect());
            // Construct a string for the diagonals
        }
        return compute_2(&matrix).try_into().unwrap();
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

fn line_to_str(vec: &Vec<&char>) -> String {
    return vec.iter().map(|c| c.to_string()).collect();
}

fn count_words(str: &str) -> usize {
    // We use two regex because we allow overlapping XMASAMX
    let xmas: Regex = Regex::new(r"XMAS").unwrap();
    let samx: Regex = Regex::new(r"SAMX").unwrap();
    return xmas.captures_iter(str).count() + samx.captures_iter(str).count();
}

fn compute_1(matrix: &Vec<Vec<char>>) -> usize {
    let count = vec![
        diagonal_pos_neg(matrix),
        diagonal_pos_pos(matrix),
        straight_x(matrix),
        straight_y(matrix),
    ]
    // Flatten the array by one level
    // eg: [['a','b'], ['c']] => ['a','b','c']
    .concat()
    .into_iter()
    // Keep only elements of length 4
    // Transform char arrays into string and check if XMAS|SAMX
    .map(|line| count_words(&line_to_str(&line)))
    // Count
    .reduce(|prev, next| prev + next)
    .unwrap();
    return count;
}

fn sub_x(matrix: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> (String, String) {
    static SIZE: usize = 3;
    // Negative slop (\)
    let mut neg: String = String::new();
    for i in 0..SIZE {
        neg.push(*matrix.get(start_x + i).unwrap().get(start_y + i).unwrap());
    }
    // Positive slop (/)
    let mut pos: String = String::new();
    for i in 0..SIZE {
        pos.push(
            *matrix
                .get(start_x + (SIZE - 1) - i)
                .unwrap()
                .get(start_y + i)
                .unwrap(),
        );
    }

    return (neg, pos);
}

fn compute_2(matrix: &Vec<Vec<char>>) -> usize {
    static CROSS_PAD: usize = 1;
    let mas: Regex = Regex::new(r"MAS|SAM").unwrap();
    let n: usize = matrix.len();
    let mut count: usize = 0;
    for i in 0 + CROSS_PAD..n - CROSS_PAD {
        for j in 0 + CROSS_PAD..n - CROSS_PAD {
            let c: &char = matrix.get(i).unwrap().get(j).unwrap();
            // Center of the X-MAS
            if *c == 'A' {
                // Retrieve both diagonals
                let (pos, neg): (String, String) = sub_x(matrix, i - 1, j - 1);
                // If both are MAS|SAM, count this X-MAS
                if mas.is_match(&pos) && mas.is_match(&neg) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day04.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i32 = 18;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i32 = 9;
        assert_eq!(result, expected);
    }
}
