fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(str::trim)
        .flat_map(str::parse::<i64>)
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let data = parse_input(input);
    //Find descending
    data.windows(2).filter(|x| x[0] < x[1]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let data = parse_input(input);
    data.windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "199
200
208
210
200
207
240
269
260
263"
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(get_test_input()), 7);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(get_test_input()), 5);
    }
}
