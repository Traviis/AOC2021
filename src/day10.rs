#[aoc_generator(day10)]
fn day10_parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn inverse_char(c: char) -> char {
    match c {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        '>' => '<',
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => panic!(),
    }
}
#[aoc(day10, part1)]
pub fn day10_part1(lines: &Vec<Vec<char>>) -> u128 {
    let mut bad_chars = vec![];

    'skip: for line in lines.iter() {
        let mut stack = vec![];
        'line: for (idx, c) in line.iter().enumerate() {
            match c {
                '(' | '[' | '<' | '{' => stack.push(c),
                ')' | ']' | '>' | '}' => {
                    match stack.pop() {
                        Some(e) => {
                            if *e == inverse_char(*c) {
                                //Great
                            } else {
                                //Syntax error, mis matched
                                //println!("{:?}", stack);
                                //println!(
                                //    "Syntax Error: {} char {} => {}",
                                //    line.iter().collect::<String>(),
                                //    idx,
                                //    line.iter().take(idx + 1).collect::<String>()
                                //);
                                bad_chars.push(*c);
                                break 'line;
                            }
                        }
                        None => break 'line, //Actually technically bad, since it means you have trailing chars
                    }
                }
                _ => panic!("Bad char: {}", c.clone()),
            }
        }
    }

    bad_chars
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unknown {}", c.clone()),
        })
        .sum::<u128>()
}
#[aoc(day10, part2)]
pub fn day10_part2(lines: &Vec<Vec<char>>) -> u128 {
    let mut sums = vec![];

    for line in lines.iter() {
        //println!("Eval: {}", line.iter().collect::<String>());
        let mut stack = vec![];
        'line: for (idx, c) in line.iter().enumerate() {
            match c {
                '(' | '[' | '<' | '{' => stack.push(c),
                ')' | ']' | '>' | '}' => {
                    match stack.pop() {
                        Some(e) => {
                            if *e == inverse_char(*c) {
                                //Great
                                //println!("Yea {:?}", stack);
                            } else {
                                //println!("Corrupt line: {}", line.iter().collect::<String>());
                                stack.clear();
                                break 'line;
                            }
                        }
                        None => {
                            break 'line;
                        } //Incomplete
                    }
                }
                _ => panic!("Bad char: {}", c.clone()),
            }
        } //end line loop
        if stack.iter().count() > 0 {
            let add_line = stack
                .iter()
                .rev()
                .map(|&x| inverse_char(*x))
                .collect::<String>();
            sums.push(
                add_line
                    .chars()
                    .fold(0, |a, cv| (a * 5) + get_complete_points(cv)),
            );
        }
    }
    sums.sort();
    let sums_count = sums.iter().count();
    //println!("Sums: {:?}", sums);
    sums[sums_count / 2]
}

fn get_complete_points(c: char) -> u128 {
    //Could inline this, looks nicer seperate
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_short_test_input() -> &'static str {
        "{([(<{}[<>[]}>{[]{[(<()>"
    }
    fn get_short_incomplete_test_input() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>"
    }

    fn get_test_input() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn day10_incomplete_parser_test() {
        assert_eq!(
            super::day10_part1(&day10_parse(get_short_incomplete_test_input())),
            0
        );
    }
    #[test]
    fn day10_parser_test() {
        assert_eq!(
            super::day10_part1(&day10_parse(get_short_test_input())),
            1197
        );
    }

    #[test]
    fn day10_part2_incomplete_single() {
        assert_eq!(
            super::day10_part2(&day10_parse(get_short_incomplete_test_input())),
            288957
        );
    }

    #[test]
    fn day10_part1() {
        assert_eq!(super::day10_part1(&day10_parse(get_test_input())), 26397);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(super::day10_part2(&day10_parse(get_test_input())), 288957);
    }
}
