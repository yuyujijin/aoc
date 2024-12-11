use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Matrix {
    values: Vec<Vec<char>>,
}

fn from_lines(lines: Lines<BufReader<File>>) -> Matrix {
    let mut matrix: Matrix = Matrix {
        values: Vec::<Vec<char>>::new(),
    };
    for line in lines.flatten() {
        matrix.values.push(line.chars().collect());
    }
    return matrix;
}

pub fn from_file(path: &str) -> Option<Matrix> {
    if let Ok(lines) = read_lines(path) {
        return Some(from_lines(lines));
    }
    return None;
}

impl Matrix {
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        return self.values.get(y).and_then(|row| row.get(x));
    }

    pub fn width(&self) -> Option<usize> {
        self.values.get(0).and_then(|row| Some(row.iter().count()))
    }

    pub fn height(&self) -> usize {
        self.values.len()
    }

    pub fn pretty_print<F>(&self, interests: Option<&Vec<(HashSet<(usize, usize)>, F)>>)
    where
        F: Fn(char) -> String,
    {
        let (width, height): (usize, usize) = (self.width().unwrap(), self.height());
        for i in 0..height {
            for j in 0..width {
                let c = self.get(j, i).unwrap();
                // If any interests provided
                match interests {
                    // Find if any of the interests contains the provided pos
                    Some(interests) => {
                        match interests.iter().find(|(pos, _)| pos.contains(&(j, i))) {
                            // If so, apply transform method
                            Some((_, f)) => print!("{}", f(*c)),
                            _ => print!("{}", c),
                        }
                    }
                    _ => print!("{}", c),
                }
            }
            println!();
        }
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        self.values
            .iter()
            .map(|row| row.iter().collect::<String>() + "\n")
            .collect()
    }
}
