use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/day06.txt";

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl fmt::Display for Direction {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Top => write!(f, "Top"),
            Direction::Right => write!(f, "Right"),
            Direction::Bottom => write!(f, "Bottom"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

fn char_to_direction(c: &char) -> Direction {
    match c {
        '^' => Direction::Top,
        '>' => Direction::Right,
        'v' => Direction::Bottom,
        '<' => Direction::Left,
        _ => panic!("Unknown direction"),
    }
}

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
    let start_chars: Vec<char> = vec!['^', '>', 'v', '<'];
    let mut matrix: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut start_dir: Option<Direction> = None;
    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.flatten().enumerate() {
            matrix.push(line.chars().collect());
            // Find the start pos
            for (j, c) in line.chars().enumerate() {
                if start_chars.contains(&c) {
                    start_pos = (j, i);
                    start_dir = Some(char_to_direction(&c));
                }
            }
        }
        return find_path(
            &matrix,
            start_pos,
            start_dir.unwrap(),
            &mut HashSet::<(usize, usize)>::new(),
        )
        .try_into()
        .unwrap();
    }
    return -1;
}

fn part2(path: &str) -> i32 {
    let start_chars: Vec<char> = vec!['^', '>', 'v', '<'];
    let mut matrix: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut start_dir: Option<Direction> = None;
    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.flatten().enumerate() {
            matrix.push(line.chars().collect());
            // Find the start pos
            for (j, c) in line.chars().enumerate() {
                if start_chars.contains(&c) {
                    start_pos = (j, i);
                    start_dir = Some(char_to_direction(&c));
                }
            }
        }
        let mut visited: HashSet<(usize, usize, Direction)> =
            HashSet::<(usize, usize, Direction)>::new();
        let dir = start_dir.unwrap();
        visited.insert((start_pos.1, start_pos.0, dir));
        return find_loops(&matrix, start_pos, dir, &mut visited)
            .try_into()
            .unwrap();
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

fn find_path(
    matrix: &Vec<Vec<char>>,
    pos: (usize, usize),
    dir: Direction,
    mut visited: &mut HashSet<(usize, usize)>,
) -> usize {
    //
    let mut count: usize = 0;
    //
    let (x, y): (usize, usize) = pos;

    let width: usize = matrix.get(0).unwrap().len();
    let height: usize = matrix.len();
    match dir {
        Direction::Top => {
            // Walk until you can't no more
            for i in (0..y).rev() {
                match matrix.get(i).unwrap().get(x).unwrap() {
                    '#' => {
                        return count
                            + find_path(&matrix, (x, i + 1), Direction::Right, &mut visited)
                    }
                    _ => match visited.get(&(x, i)) {
                        Some(_) => (),
                        // If not visited already, mark it as visited
                        _ => {
                            visited.insert((x, i));
                            count += 1;
                        }
                    },
                }
            }
            // Reached an end
            return count;
        }
        Direction::Right => {
            // Walk until you can't no more
            for i in x..width {
                match matrix.get(y).unwrap().get(i).unwrap() {
                    '#' => {
                        return count
                            + find_path(&matrix, (i - 1, y), Direction::Bottom, &mut visited)
                    }
                    _ => match visited.get(&(i, y)) {
                        Some(_) => (),
                        // If not visited already, mark it as visited
                        _ => {
                            visited.insert((i, y));
                            count += 1;
                        }
                    },
                }
            }
            // Reached an end
            return count;
        }
        Direction::Bottom => {
            // Walk until you can't no more
            for i in y..height {
                match matrix.get(i).unwrap().get(x).unwrap() {
                    '#' => {
                        return count
                            + find_path(&matrix, (x, i - 1), Direction::Left, &mut visited)
                    }
                    _ => match visited.get(&(x, i)) {
                        Some(_) => (),
                        // If not visited already, mark it as visited
                        _ => {
                            visited.insert((x, i));
                            count += 1;
                        }
                    },
                }
            }
            // Reached an end
            return count;
        }
        Direction::Left => {
            // Walk until you can't no more
            for i in (0..x).rev() {
                match matrix.get(y).unwrap().get(i).unwrap() {
                    '#' => {
                        return count + find_path(&matrix, (i + 1, y), Direction::Top, &mut visited)
                    }
                    _ => match visited.get(&(i, y)) {
                        Some(_) => (),
                        // If not visited already, mark it as visited
                        _ => {
                            visited.insert((i, y));
                            count += 1;
                        }
                    },
                }
            }
            // Reached an end
            return count;
        }
    }
}

fn find_loops(
    matrix: &Vec<Vec<char>>,
    pos: (usize, usize),
    dir: Direction,
    mut visited: &mut HashSet<(usize, usize, Direction)>,
) -> usize {
    //
    let mut count: usize = 0;
    //
    let (x, y): (usize, usize) = pos;
    visited.insert((x, y, dir));

    // println!(
    // "{}",
    // &visited
    // .iter()
    // .map(|(x, y, dir)| format!("({}, {}, {})", x, y, dir))
    // .reduce(|a, b| a + "," + &b)
    // .unwrap()
    // );

    let width: usize = matrix.get(0).unwrap().len();
    let height: usize = matrix.len();
    match dir {
        Direction::Top => {
            // Walk until you can't no more
            for i in (0..y).rev() {
                match matrix.get(i).unwrap().get(x).unwrap() {
                    '#' => {
                        return count
                            + find_loops(&matrix, (x, i + 1), Direction::Right, &mut visited)
                    }
                    // Visited +90°
                    _ => {
                        println!("({}, {}), {}", x, i, dir);
                        match visited.get(&(x, i, Direction::Right)) {
                            // This is a valid boulder place
                            Some(_) => {
                                println!("({}, {}) {} already visited !", x, i, Direction::Right);
                                count += 1;
                            }
                            _ => (),
                        }
                        visited.insert((x, i, dir));
                    }
                }
            }
            // Reached an end
            return count;
        }
        Direction::Right => {
            // Walk until you can't no more
            for i in x..width {
                match matrix.get(y).unwrap().get(i).unwrap() {
                    '#' => {
                        return count
                            + find_loops(&matrix, (i - 1, y), Direction::Bottom, &mut visited)
                    }
                    // Visited +90°
                    _ => {
                        println!("({}, {}), {}", i, y, dir);
                        match visited.get(&(i, y, Direction::Bottom)) {
                            // This is a valid boulder place
                            Some(_) => {
                                println!("({}, {}) {} already visited !", i, y, Direction::Bottom);
                                count += 1;
                            }
                            _ => (),
                        };
                        visited.insert((i, y, dir));
                    }
                }
            }
            // Reached an end
            return count;
        }
        Direction::Bottom => {
            // Walk until you can't no more
            for i in y..height {
                match matrix.get(i).unwrap().get(x).unwrap() {
                    '#' => {
                        return count
                            + find_loops(&matrix, (x, i - 1), Direction::Left, &mut visited)
                    }
                    // Visited +90°
                    _ => {
                        println!("({}, {}), {}", x, i, dir);
                        match visited.get(&(x, i, Direction::Left)) {
                            // This is a valid boulder place
                            Some(_) => {
                                println!("({}, {}) {} already visited !", x, i, Direction::Left);
                                count += 1;
                            }
                            _ => (),
                        };
                        visited.insert((x, i, dir));
                    }
                }
            }
            // Reached an end
            return count;
        }
        Direction::Left => {
            // Walk until you can't no more
            for i in (0..x).rev() {
                match matrix.get(y).unwrap().get(i).unwrap() {
                    '#' => {
                        return count
                            + find_loops(&matrix, (i + 1, y), Direction::Top, &mut visited)
                    }
                    // Visited +90°
                    _ => {
                        println!("({}, {}), {}", i, y, dir);
                        match visited.get(&(i, y, Direction::Top)) {
                            // This is a valid boulder place
                            Some(_) => {
                                println!("({}, {}) {} already visited !", i, y, Direction::Top);
                                count += 1;
                            }
                            _ => (),
                        };
                        visited.insert((i, y, dir));
                    }
                }
            }
            // Reached an end
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_FILE_PATH: &str = "inputs/day06.example.txt";

    #[test]
    fn example_1() {
        let result = part1(EXAMPLE_FILE_PATH);
        let expected: i32 = 41;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i32 = 6;
        assert_eq!(result, expected);
    }
}
