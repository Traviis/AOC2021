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
    /*
    let mut counts = Vec::new();
    for _ in 0..data[0].chars().count() {
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
    */

    let mut o2 = data.clone();
    let mut co2 = data.clone();

    for c_idx in 0..data[0].chars().count() {
        //Recount only the remaining

        if o2.iter().count() != 1 {
            let mut o2_zeroes = 0;
            let mut o2_ones = 0;

            //TODO, do this using fold into a tuple
            o2.iter().for_each(|x| {
                if x.chars().nth(c_idx).unwrap() == '0' {
                    o2_zeroes += 1;
                } else {
                    o2_ones += 1;
                }
            });

            o2 = o2
                .into_iter()
                .filter(|x| {
                    //println!("{} {} {}", c_idx, o2_zeroes, o2_ones);
                    let keep_value = if o2_zeroes <= o2_ones { '1' } else { '0' };
                    keep_value == x.chars().nth(c_idx).unwrap()
                })
                .collect::<Vec<&str>>();
            //This is super inefficient and lazy, since we are splitting it up every run
            //Fix this, but for now, go ahead and get it right
        }
        //println!("o2 {:?}", o2);

        if co2.iter().count() != 1 {
            let mut co2_zeroes = 0;
            let mut co2_ones = 0;

            co2.iter().for_each(|x| {
                if x.chars().nth(c_idx).unwrap() == '0' {
                    co2_zeroes += 1;
                } else {
                    co2_ones += 1;
                }
            });

            co2 = co2
                .into_iter()
                .filter(|x| {
                    //Keep 0th if match
                    //Otherwise least common
                    println!("{} {} {}", c_idx, co2_zeroes, co2_ones);
                    let keep_value = if co2_zeroes <= co2_ones { '0' } else { '1' };
                    keep_value == x.chars().nth(c_idx).unwrap()
                })
                .collect::<Vec<&str>>();
            //This is super inefficient and lazy, since we are splitting it up every run
            println!("co2 {:?}", co2);
        }
    }
    let o2_val = usize::from_str_radix(&o2.iter().next().unwrap(), 2).unwrap();
    let co2_val = usize::from_str_radix(&co2.iter().next().unwrap(), 2).unwrap();

    o2_val * co2_val
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
