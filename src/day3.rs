fn parse_input(input: &str) -> Vec<&str> {
    input.lines().map(str::trim).collect::<Vec<_>>()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let data = parse_input(input);
    let mut counts = Vec::new();
    for i in 0..data[0].chars().count() {
        counts.push((0, 0));
    }

    data.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => counts[i].0 = counts[i].0 + 1,
                '1' => counts[i].1 = counts[i].1 + 1,
                _ => panic!("Invalid value"),
            }
        }
    });
    println!("{:?}", counts);
    let gamma_string = counts
        .iter()
        .map(|(z, o)| if z > o { '0' } else { '1' })
        .collect::<String>();
    //Yes, I'm sure there is a clever way to do this, but I'm lazy right now
    let epsilon_string = counts
        .iter()
        .map(|(z, o)| if z > o { '1' } else { '0' })
        .collect::<String>();

    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_string, 2).unwrap();
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let data = parse_input(input);
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(get_test_input()), 198);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(get_test_input()), 5);
    }
}
