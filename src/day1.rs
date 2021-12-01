fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(str::trim)
        .flat_map(str::parse::<i64>)
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let data = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    //use super::{part1_chars as part1, part2};

    #[test]
    fn example_part1() {}
}
