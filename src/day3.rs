fn parse_input(input: &str) -> Vec<&str> {
    input.lines().map(str::trim).collect::<Vec<_>>()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let data = parse_input(input);
    let mut counts = Vec::new();
    for i in 0..data[0].chars().count() {
        let (z, o) = get_count_for_index(&data, i);
        counts.push((z, o));
    }

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

    let mut o2 = data.clone();
    let mut co2 = data.clone();

    for c_idx in 0..data[0].chars().count() {
        //Recount only the remaining

        if o2.iter().count() != 1 {
            let (z, o) = get_count_for_index(&o2, c_idx);

            o2 = o2
                .into_iter()
                .filter(|x| {
                    //println!("{} {} {}", c_idx, o2_zeroes, o2_ones);
                    let keep_value = if z <= o { '1' } else { '0' };
                    keep_value == x.chars().nth(c_idx).unwrap()
                })
                .collect::<Vec<&str>>();
        }

        if co2.iter().count() != 1 {
            let (z, o) = get_count_for_index(&co2, c_idx);

            co2 = co2
                .into_iter()
                .filter(|x| {
                    //Keep 0th if match
                    //Otherwise least common
                    let keep_value = if z <= o { '0' } else { '1' };
                    keep_value == x.chars().nth(c_idx).unwrap()
                })
                .collect::<Vec<&str>>();
        }
    }
    let o2_val = usize::from_str_radix(&o2.iter().next().unwrap(), 2).unwrap();
    let co2_val = usize::from_str_radix(&co2.iter().next().unwrap(), 2).unwrap();

    o2_val * co2_val
}

fn get_count_for_index(vec: &Vec<&str>, idx: usize) -> (usize, usize) {
    vec.iter().fold((0, 0), |(a, b), x| {
        if x.chars().nth(idx).unwrap() == '0' {
            (a + 1, b)
        } else {
            (a, b + 1)
        }
    })
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
        assert_eq!(part2(get_test_input()), 230);
    }
}
