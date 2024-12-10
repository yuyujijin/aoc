use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{fmt, thread};

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

fn part1(path: &str) -> i64 {
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

fn part2(path: &str) -> i64 {
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
        let mut stones: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();

        // Fake insert `start_pos` (can't be a valid stone position)
        stones.insert((start_pos.0, start_pos.1));

        let start = (start_pos.0, start_pos.1, start_dir.unwrap());

        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        find_loops(&matrix, start, &mut stones, start, &tx);

        drop(tx);

        return rx
            .iter()
            .map(|x| {
                if x {
                    return 1;
                } else {
                    return 0;
                }
            })
            .reduce(|prev, next| prev + next)
            .unwrap();
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

fn set_stone(matrix: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<Vec<char>> {
    let (x, y) = pos;
    let mut cloned: Vec<Vec<char>> = matrix.clone();
    // Modify (x, y) as '#'
    *cloned.get_mut(y).unwrap().get_mut(x).unwrap() = '#';
    return cloned;
}

fn is_loop(
    matrix: &Vec<Vec<char>>,
    guard: (usize, usize, Direction),
    mut visited: &mut HashSet<(usize, usize, Direction)>,
) -> bool {
    let (x, y, dir): (usize, usize, Direction) = guard;

    let width: usize = matrix.get(0).unwrap().len();
    let height: usize = matrix.len();

    match dir {
        Direction::Top => {
            // Walk until you can't no more
            for i in (1..y + 1).rev() {
                match matrix.get(i - 1).unwrap().get(x).unwrap() {
                    '#' => {
                        return is_loop(&matrix, (x, i, Direction::Right), &mut visited);
                    }
                    _ => match visited.get(&(x, i, dir)) {
                        Some(_) => {
                            return true;
                        }
                        _ => {
                            visited.insert((x, i, dir));
                        }
                    },
                }
            }
        }
        Direction::Right => {
            // Walk until you can't no more
            for i in x..width - 1 {
                match matrix.get(y).unwrap().get(i + 1).unwrap() {
                    '#' => return is_loop(&matrix, (i, y, Direction::Bottom), &mut visited),
                    _ => match visited.get(&(i, y, dir)) {
                        Some(_) => {
                            return true;
                        }
                        _ => {
                            visited.insert((i, y, dir));
                        }
                    },
                }
            }
        }
        Direction::Bottom => {
            // Walk until you can't no more
            for i in y..height - 1 {
                match matrix.get(i + 1).unwrap().get(x).unwrap() {
                    '#' => return is_loop(&matrix, (x, i, Direction::Left), &mut visited),
                    _ => match visited.get(&(x, i, dir)) {
                        Some(_) => {
                            return true;
                        }
                        _ => {
                            visited.insert((x, i, dir));
                        }
                    },
                }
            }
        }
        Direction::Left => {
            // Walk until you can't no more
            for i in (1..x + 1).rev() {
                match matrix.get(y).unwrap().get(i - 1).unwrap() {
                    '#' => return is_loop(&matrix, (i, y, Direction::Top), &mut visited),
                    _ => match visited.get(&(i, y, dir)) {
                        Some(_) => {
                            return true;
                        }
                        _ => {
                            visited.insert((i, y, dir));
                        }
                    },
                }
            }
        }
    }
    return false;
}

fn find_loops(
    matrix: &Vec<Vec<char>>,
    guard: (usize, usize, Direction),
    mut stones: &mut HashSet<(usize, usize)>,
    start: (usize, usize, Direction),
    tx: &Sender<bool>,
) {
    let (x, y, dir): (usize, usize, Direction) = guard;

    let width: usize = matrix.get(0).unwrap().len();
    let height: usize = matrix.len();

    match dir {
        Direction::Top => {
            // Walk until you can't no more
            for i in (1..y + 1).rev() {
                match matrix.get(i - 1).unwrap().get(x).unwrap() {
                    '#' => {
                        return find_loops(
                            &matrix,
                            (x, i, Direction::Right),
                            &mut stones,
                            start,
                            &tx,
                        )
                    }
                    _ => {
                        if !stones.contains(&(x, i - 1)) {
                            let (matrix_cloned, tx_cloned) = (matrix.clone(), tx.clone());
                            thread::spawn(move || {
                                tx_cloned
                                    .clone()
                                    .send(is_loop(
                                        &set_stone(&matrix_cloned, (x, i - 1)),
                                        start,
                                        &mut HashSet::<(usize, usize, Direction)>::new(),
                                    ))
                                    .unwrap()
                            });
                            stones.insert((x, i - 1));
                        }
                    }
                }
            }
        }
        Direction::Right => {
            // Walk until you can't no more
            for i in x..width - 1 {
                match matrix.get(y).unwrap().get(i + 1).unwrap() {
                    '#' => {
                        return find_loops(
                            &matrix,
                            (i, y, Direction::Bottom),
                            &mut stones,
                            start,
                            &tx,
                        )
                    }
                    _ => {
                        if !stones.contains(&(i + 1, y)) {
                            let (matrix_cloned, tx_cloned) = (matrix.clone(), tx.clone());
                            thread::spawn(move || {
                                tx_cloned
                                    .clone()
                                    .send(is_loop(
                                        &set_stone(&matrix_cloned, (i + 1, y)),
                                        start,
                                        &mut HashSet::<(usize, usize, Direction)>::new(),
                                    ))
                                    .unwrap()
                            });
                            stones.insert((i + 1, y));
                        }
                    }
                }
            }
        }
        Direction::Bottom => {
            // Walk until you can't no more
            for i in y..height - 1 {
                match matrix.get(i + 1).unwrap().get(x).unwrap() {
                    '#' => {
                        return find_loops(
                            &matrix,
                            (x, i, Direction::Left),
                            &mut stones,
                            start,
                            &tx,
                        )
                    }
                    _ => {
                        if !stones.contains(&(x, i + 1)) {
                            let (matrix_cloned, tx_cloned) = (matrix.clone(), tx.clone());
                            thread::spawn(move || {
                                tx_cloned
                                    .clone()
                                    .send(is_loop(
                                        &set_stone(&matrix_cloned, (x, i + 1)),
                                        start,
                                        &mut HashSet::<(usize, usize, Direction)>::new(),
                                    ))
                                    .unwrap()
                            });
                            stones.insert((x, i + 1));
                        }
                    }
                }
            }
        }
        Direction::Left => {
            // Walk until you can't no more
            for i in (1..x + 1).rev() {
                match matrix.get(y).unwrap().get(i - 1).unwrap() {
                    '#' => {
                        return find_loops(&matrix, (i, y, Direction::Top), &mut stones, start, &tx)
                    }
                    _ => {
                        if !stones.contains(&(i - 1, y)) {
                            let (matrix_cloned, tx_cloned) = (matrix.clone(), tx.clone());
                            thread::spawn(move || {
                                tx_cloned
                                    .send(is_loop(
                                        &set_stone(&matrix_cloned, (i - 1, y)),
                                        start,
                                        &mut HashSet::<(usize, usize, Direction)>::new(),
                                    ))
                                    .unwrap()
                            });
                            stones.insert((i - 1, y));
                        }
                    }
                }
            }
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
        let expected: i64 = 41;
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let result = part2(EXAMPLE_FILE_PATH);
        let expected: i64 = 6;
        assert_eq!(result, expected);
    }
}
