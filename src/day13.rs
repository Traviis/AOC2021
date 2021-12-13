//use std::collections::HashMap;

#[aoc_generator(day13)]
fn day13_parse(input: &str) -> &str {
    ""
}

#[aoc(day13, part1)]
pub fn day13_part1(map: &str) -> u128 {
    0
}
#[aoc(day13, part2)]
pub fn day13_part2(map: &str) -> u128 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7"
    }

    #[test]
    fn day13_part1() {
        assert_eq!(super::day13_part1(&day13_parse(get_test_input())), 17);
    }

    #[test]
    fn day13_part2() {
        assert_eq!(super::day13_part2(&day13_parse(get_test_input())), 36);
    }
}
