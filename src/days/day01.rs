fn part1() -> String {
    return String::from("part1");
}

fn part2() -> String {
    return String::from("part2");
}

pub fn solve(part: i32) -> String {
    match part {
        1 => part1(),
        2 => part2(),
        _ => unimplemented!(),
    }
}
