use std::fmt;
use std::str::FromStr;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

struct Pos {
    hor: i64,
    vert: i64,
    aim: i64,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (l, v) = match self {
            Command::Forward(n) => ("F", n),
            Command::Down(n) => ("D", n),
            Command::Up(n) => ("U", n),
        };
        write!(f, "{} [{}]", l, v)
    }
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut strs = s.split(" ");
        let cmd = strs.next().ok_or("Incomplete Command (op)")?;
        let val = strs
            .next()
            .ok_or("Incomplete Command (value)")?
            .parse::<i64>()?;
        match cmd {
            "forward" => Ok(Command::Forward(val)),
            "down" => Ok(Command::Down(val)),
            "up" => Ok(Command::Up(val)),
            _ => Err("Unknown Command".into()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(str::trim)
        .flat_map(str::parse::<Command>)
        .collect::<Vec<_>>()
}

#[aoc(day2, part1, obj)]
pub fn part1(input: &str) -> i64 {
    let data = parse_input(input);

    //println!("Size: {}", data.len());
    //data.iter().for_each(|x| println!("{}", x));

    let mut pos = Pos {
        hor: 0,
        vert: 0,
        aim: 0,
    };
    data.iter().for_each(|x| match x {
        Command::Forward(n) => pos.hor += n,
        Command::Down(n) => pos.vert += n,
        Command::Up(n) => pos.vert -= n,
    });

    pos.vert * pos.hor
}

#[aoc(day2, part1, fold)]
pub fn part1_fold(input: &str) -> i64 {
    let data = parse_input(input);

    let sum = data.iter().fold((0, 0), |(x, y), cmd| match cmd {
        Command::Forward(n) => (x + n, y),
        Command::Down(n) => (x, y + n),
        Command::Up(n) => (x, y - n),
    });

    sum.0 * sum.1
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i64 {
    let data = parse_input(input);
    let mut pos = Pos {
        hor: 0,
        vert: 0,
        aim: 0,
    };
    data.iter().for_each(|x| match x {
        Command::Forward(n) => {
            pos.hor += n;
            pos.vert += n * pos.aim;
        }
        Command::Down(n) => pos.aim += n,
        Command::Up(n) => pos.aim -= n,
    });

    pos.vert * pos.hor
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "forward 5
down 5
forward 8
up 3
down 8
forward 2"
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(get_test_input()), 150);
    }

    #[test]
    fn example_part1_fold() {
        assert_eq!(part1_fold(get_test_input()), 150);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(get_test_input()), 900);
    }
}
