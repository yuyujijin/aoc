use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day09.txt";

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Copy)]
enum Block {
    File(usize, usize),
    Space(usize),
}

impl ToString for Block {
    fn to_string(&self) -> String {
        match self {
            Block::File(size, index) => vec![index.to_string(); *size].concat(),
            Block::Space(size) => vec!["."; *size].concat(),
        }
    }
}

fn part1(path: &str) -> i64 {
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            return checksum(&compact(&expand(&line)));
        }
    }
    return -1;
}

fn part2(path: &str) -> i64 {
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            return block_checksum(&block_compact(&block_expand(&line)))
                .try_into()
                .unwrap();
        }
    }
    return -1;
}

fn expand(str: &str) -> Vec<Option<i64>> {
    let mut space: bool = false;
    let mut index: usize = 0;
    // Iterate through chars
    str.chars()
        // Expand the char by its given size
        // Writing its index or '.' if space
        .map(|c| {
            let size = c.to_digit(10).unwrap() as usize;
            let expanded = match space {
                true => vec![None; size],
                false => {
                    let value = vec![Some(index as i64); size];
                    index += 1;
                    value
                }
            };
            space = !space;
            return expanded;
        })
        .flatten()
        .collect()
}

fn compact(expanded: &Vec<Option<i64>>) -> Vec<Option<i64>> {
    // Size is only the number of available space
    let size = expanded.len();
    let mut compacted = expanded.clone();

    for i in (0..size).rev() {
        match compacted.get(i) {
            None => continue,
            Some(_) => {
                // Find first occurence of None
                let first_occ: usize = compacted.iter().position(|elt| elt.is_none()).unwrap();
                // Finished compacting
                if first_occ > i {
                    break;
                }
                // Swap value at index `first_occ` with value at index `i`
                compacted.swap(first_occ, i);
            }
        }
    }

    return compacted;
}

fn checksum(compacted: &Vec<Option<i64>>) -> i64 {
    compacted
        .iter()
        // Filter out dots
        .filter(|c| c.is_some())
        .enumerate()
        // Convert char to u32 and multiply them by their index
        .map(|(i, c)| c.unwrap() * (i as i64))
        // Add everything
        .reduce(|prev: i64, next: i64| prev + next)
        .unwrap()
}

fn block_expand(str: &str) -> Vec<Block> {
    let mut space: bool = false;
    let mut index: usize = 0;
    // Iterate through chars
    str.chars()
        // Expand the char by its given size
        // Writing its index or '.' if space
        .map(|c| {
            let size = c.to_digit(10).unwrap() as usize;
            let expanded = match space {
                true => Block::Space(size),
                false => {
                    let value = Block::File(size, index);
                    index += 1;
                    value
                }
            };
            space = !space;
            return expanded;
        })
        .collect()
}

fn block_compact(expanded: &Vec<Block>) -> Vec<Block> {
    // Size is only the number of available space
    let size = expanded.len();
    let mut compacted = expanded.clone();

    for i in (0..size).rev() {
        match compacted.get(i).cloned() {
            // Find the rightest most block
            Some(Block::File(file_size, _)) => {
                // Iterate through empty spaces and try to insert our file into available space
                for j in 0..i {
                    match compacted.get(j).cloned() {
                        Some(Block::Space(empty_size)) => {
                            // It can fit !
                            if empty_size >= file_size {
                                // Swap both blocks
                                compacted.swap(i, j);
                                // If empty_size is larger than file_size
                                // ex: Space(20), ..., File(12) => File(12) + Space(8), ..., Space(12)
                                if empty_size > file_size {
                                    // * We have to remove some Space space to swapped Space space
                                    compacted.drain(i..i + 1);
                                    compacted.insert(i, Block::Space(file_size));
                                    // * We have to insert back some Space space
                                    let delta = empty_size - file_size;
                                    compacted.insert(j + 1, Block::Space(delta));
                                }
                                // Stop looking for empty space
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => continue,
        }
    }
    return compacted;
}

fn block_checksum(compacted: &Vec<Block>) -> usize {
    let mut index: usize = 0;
    compacted
        .iter()
        .map(|block| match block {
            Block::File(size, file_index) => {
                // Iterate on block to calculate its full size
                let value = (index..index + size)
                    .map(|n| file_index * n)
                    .reduce(|prev, next| prev + next)
                    .unwrap();
                // Increment by file size
                index += size;
                return value;
            }
            Block::Space(size) => {
                // Increment by space size
                index += size;
                // Empty so worth zero
                return 0;
            }
        })
        .reduce(|prev: usize, next: usize| prev + next)
        .unwrap()
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

    static EXAMPLE_FILE_PATH: &str = "inputs/day09.example.txt";

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
