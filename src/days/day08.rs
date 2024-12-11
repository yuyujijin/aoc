use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::utils::matrix::{from_file, Matrix};

static INPUT_FILE_PATH: &str = "inputs/day08.txt";

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
    let mut antennas: HashMap<char, Vec<(usize, usize)>> =
        HashMap::<char, Vec<(usize, usize)>>::new();
    let matrix: Matrix = from_file(path).unwrap();
    let (width, height) = (matrix.width().unwrap(), matrix.height());

    for i in 0..height {
        for j in 0..width {
            match matrix.get(j, i).unwrap() {
                '.' => (),
                c => {
                    antennas
                        .entry(*c)
                        .or_insert(Vec::<(usize, usize)>::new())
                        .push((j, i));
                }
            }
        }
    }
    let antinodes: HashSet<(usize, usize)> = calculate_antinodes(&antennas, width, height);
    return antinodes.len().try_into().unwrap();
}

fn part2(path: &str) -> i64 {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> =
        HashMap::<char, Vec<(usize, usize)>>::new();
    let matrix: Matrix = from_file(path).unwrap();
    let (width, height) = (matrix.width().unwrap(), matrix.height());

    for i in 0..height {
        for j in 0..width {
            match matrix.get(j, i).unwrap() {
                '.' => (),
                c => {
                    antennas
                        .entry(*c)
                        .or_insert(Vec::<(usize, usize)>::new())
                        .push((j, i));
                }
            }
        }
    }
    let antinodes: HashSet<(usize, usize)> =
        calculate_antinodes_fixed_point(&antennas, width, height);

    return antinodes.len().try_into().unwrap();
}

fn calculate_symetries(
    a: &(usize, usize),
    b: &(usize, usize),
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut symetries: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();

    // Retrieve coordinates
    let (x_a, y_a) = *a;
    let (x_b, y_b) = *b;

    // Out-of-bounds
    if x_b <= 2 * x_a && 2 * x_a - x_b < width && y_b <= 2 * y_a && 2 * y_a - y_b < height {
        symetries.insert((2 * x_a - x_b, 2 * y_a - y_b));
    }
    if x_a <= 2 * x_b && 2 * x_b - x_a < width && y_a <= 2 * y_b && 2 * y_b - y_a < height {
        symetries.insert((2 * x_b - x_a, 2 * y_b - y_a));
    }

    return symetries;
}

fn calculate_antinode(
    antennas: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();
    let size = antennas.len();
    for i in 0..size {
        for j in i + 1..size {
            let (head, elt) = (antennas.get(i).unwrap(), antennas.get(j).unwrap());
            // antinodes U symetries
            antinodes.extend(&calculate_symetries(head, elt, width, height));
        }
    }
    return antinodes;
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();
    for (_, v) in antennas.iter() {
        antinodes.extend(&calculate_antinode(&v, width, height));
    }
    return antinodes;
}

fn calculate_antinode_fixed_point(
    antennas: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();
    let size = antennas.len();

    for i in 0..size {
        for j in i + 1..size {
            let (head, elt) = (antennas.get(i).unwrap(), antennas.get(j).unwrap());

            // Create the starting antinodes
            let mut computed: HashSet<(usize, usize)> =
                calculate_antinode(&[*head, *elt].to_vec(), width, height);
            let mut size = computed.len();

            // Iterate until fixed point
            loop {
                // Create a vector from all the antinodes with the two antennas
                let v: Vec<(usize, usize)> =
                    [[*head, *elt].to_vec(), Vec::from_iter(computed.clone())].concat();

                // Compute the antinode with this vectors, and UNION it with computeds
                computed.extend(calculate_antinode(&v, width, height));

                // If fixed point is reached, stop
                if computed.len() == size {
                    break;
                }
                size = computed.len();
            }

            antinodes.extend(&computed);
            // Don't forger the antennas themselves ! /!\
            antinodes.insert(*head);
            antinodes.insert(*elt);
        }
    }
    return antinodes;
}

fn calculate_antinodes_fixed_point(
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();
    for (_, v) in antennas.iter() {
        antinodes.extend(&calculate_antinode_fixed_point(&v, width, height));
    }
    return antinodes;
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

    static EXAMPLE_FILE_PATH: &str = "inputs/day08.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i64 = 14;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = 34;
        assert_eq!(result, expected);
    }
}
