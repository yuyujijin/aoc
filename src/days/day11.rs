use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day11.txt";

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
    if let Ok(lines) = read_lines(path) {
        let mut stones_map: HashMap<String, i64> = HashMap::<String, i64>::new();
        let line: String = lines.flatten().collect();
        // Insert default values for stones
        line.split_whitespace()
            .map(|s| s.to_string())
            .for_each(|stone| *stones_map.entry(stone).or_default() += 1);
        // Iterate 25 times
        for _ in 0..25 {
            stones_map = blink_map(&stones_map);
        }
        // Compute the result
        return stones_map
            .values()
            .cloned()
            .reduce(|prev, next| prev + next)
            .unwrap();
    }
    return -1;
}

fn part2(path: &str) -> i64 {
    if let Ok(lines) = read_lines(path) {
        let mut stones_map: HashMap<String, i64> = HashMap::<String, i64>::new();
        let line: String = lines.flatten().collect();
        // Insert default values for stones
        line.split_whitespace()
            .map(|s| s.to_string())
            .for_each(|stone| *stones_map.entry(stone).or_default() += 1);
        // Iterate 75 times
        for _ in 0..75 {
            stones_map = blink_map(&stones_map);
        }
        // Compute the result
        return stones_map
            .values()
            .cloned()
            .reduce(|prev, next| prev + next)
            .unwrap();
    }
    return -1;
}

fn blink(stone: &String) -> Vec<String> {
    if stone.parse::<i64>().unwrap() == 0 {
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1

        return vec![String::from("1")];
    } else if stone.len() % 2 == 0 {
        // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

        let length: usize = stone.len();
        let (left, right) = (
            stone[0..length / 2].to_string(),
            // Parsed to `i64` then back to `String` to remove zeros
            stone[length / 2..]
                .to_string()
                .parse::<i64>()
                .unwrap()
                .to_string(),
        );
        // Remove index `i`
        // Insert `left` & `right`
        return vec![left, right];
    } else {
        // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone

        return vec![(stone.parse::<i64>().unwrap() * 2024).to_string()];
    }
}

fn blink_map(stones: &HashMap<String, i64>) -> HashMap<String, i64> {
    let mut output: HashMap<String, i64> = HashMap::<String, i64>::new();
    for (n, v) in stones.into_iter() {
        let blinkeds = blink(n);
        for stone in blinkeds {
            *output.entry(stone).or_default() += *v;
        }
    }
    return output;
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

    static EXAMPLE_FILE_PATH: &str = "inputs/day11.example.txt";

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
