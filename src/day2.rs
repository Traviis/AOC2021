use std::num::ParseIntError;
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

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut strs = s.split(" ");
        match strs.clone().nth(0).unwrap() {
            "forward" => Ok(Command::Forward(strs.nth(1).unwrap().parse::<i64>()?)),
            "down" => Ok(Command::Down(strs.nth(1).unwrap().parse::<i64>()?)),
            "up" => Ok(Command::Up(strs.nth(1).unwrap().parse::<i64>()?)),
            _ => panic!("Unknown command"),
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

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    let data = parse_input(input);
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
    fn example_part2() {
        assert_eq!(part2(get_test_input()), 900);
    }
}
