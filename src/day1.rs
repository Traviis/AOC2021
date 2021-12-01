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
    //Find descending
    data.windows(2).filter(|x| x[0] < x[1]).count() as i32
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let data = parse_input(input);
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_part1() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(super::part1(input), 7);
    }

    #[test]
    fn example_part2() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(super::part2(input), 5);
    }
}
