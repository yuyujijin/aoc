use std::collections::HashSet;

use crate::utils::matrix::{from_file, Matrix};

static INPUT_FILE_PATH: &str = "inputs/day10.txt";

fn part1(path: &str) -> i64 {
    match from_file(path) {
        Some(matrix) => {
            let mut count = 0;
            for i in 0..matrix.height() {
                for j in 0..matrix.width().unwrap() {
                    let c: &char = matrix.get(j, i).unwrap();
                    if *c == '0' {
                        count += walk(
                            &matrix,
                            (j, i),
                            &mut HashSet::<(usize, usize)>::new(),
                            &mut HashSet::<(usize, usize)>::new(),
                        );
                    }
                }
            }
            count
        }
        None => -1,
    }
}

fn part2(path: &str) -> i64 {
    match from_file(path) {
        Some(matrix) => {
            let mut count = 0;
            for i in 0..matrix.height() {
                for j in 0..matrix.width().unwrap() {
                    let c: &char = matrix.get(j, i).unwrap();
                    if *c == '0' {
                        count +=
                            walk_rating(&matrix, (j, i), &mut HashSet::<(usize, usize)>::new());
                    }
                }
            }
            count
        }
        None => -1,
    }
}

pub fn solve(part: i64) -> i64 {
    match part {
        1 => part1(INPUT_FILE_PATH),
        2 => part2(INPUT_FILE_PATH),
        _ => unimplemented!(),
    }
}

fn walk(
    matrix: &Matrix,
    position: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    mut trailheads: &mut HashSet<(usize, usize)>,
) -> i64 {
    // Already visited -> exit
    if visited.contains(&position) {
        return 0;
    }
    // Mark as visited
    visited.insert(position);
    let (x, y): (usize, usize) = position;
    let value = matrix.get(x, y).unwrap().to_digit(10).unwrap();
    // Reached a peak (once)
    if value == 9 && !trailheads.contains(&position) {
        // Insert it into trailheads
        trailheads.insert(position);
        return 1;
    }
    let mut peaks: i64 = 0;
    // Walk top
    if y > 0 && matrix.get(x, y - 1).unwrap().to_digit(10).unwrap() == value + 1 {
        peaks += walk(&matrix, (x, y - 1), &mut visited.clone(), &mut trailheads);
    }
    // Walk right
    if x < matrix.width().unwrap() - 1
        && matrix.get(x + 1, y).unwrap().to_digit(10).unwrap() == value + 1
    {
        peaks += walk(&matrix, (x + 1, y), &mut visited.clone(), &mut trailheads);
    }
    // Walk bottom
    if y < matrix.height() - 1 && matrix.get(x, y + 1).unwrap().to_digit(10).unwrap() == value + 1 {
        peaks += walk(&matrix, (x, y + 1), &mut visited.clone(), &mut trailheads);
    }
    // Walk left
    if x > 0 && matrix.get(x - 1, y).unwrap().to_digit(10).unwrap() == value + 1 {
        peaks += walk(&matrix, (x - 1, y), &mut visited.clone(), &mut trailheads);
    }
    peaks
}

fn walk_rating(
    matrix: &Matrix,
    position: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> i64 {
    // Already visited -> exit
    if visited.contains(&position) {
        return 0;
    }
    // Mark as visited
    visited.insert(position);
    let (x, y): (usize, usize) = position;
    let value = matrix.get(x, y).unwrap().to_digit(10).unwrap();
    // Reached a peak (once)
    if value == 9 {
        // Insert it into trailheads
        return 1;
    }
    let mut peaks: i64 = 0;
    // Walk top
    if y > 0 && matrix.get(x, y - 1).unwrap().to_digit(10).unwrap() == value + 1 {
        peaks += walk_rating(&matrix, (x, y - 1), &mut visited.clone());
    }
    // Walk right
    if x < matrix.width().unwrap() - 1
        && matrix.get(x + 1, y).unwrap().to_digit(10).unwrap() == value + 1
    {
        peaks += walk_rating(&matrix, (x + 1, y), &mut visited.clone());
    }
    // Walk bottom
    if y < matrix.height() - 1 && matrix.get(x, y + 1).unwrap().to_digit(10).unwrap() == value + 1 {
        peaks += walk_rating(&matrix, (x, y + 1), &mut visited.clone());
    }
    // Walk left
    if x > 0 && matrix.get(x - 1, y).unwrap().to_digit(10).unwrap() == value + 1 {
        peaks += walk_rating(&matrix, (x - 1, y), &mut visited.clone());
    }
    peaks
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day10.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i64 = 36;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = 81;
        assert_eq!(result, expected);
    }
}
