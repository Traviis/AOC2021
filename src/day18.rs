use std::str::FromStr;

pub enum SNumber {
    Pair(Box<SNumber>,Box<SNumber>),
    Lit(u64)
}

impl FromStr for SNumber {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //Iterate over chars, if you see a `[` then it's the start of a new Snumber, if it's a
        //number literal, then it's part of the SNumber, if it's another `[` it's the start of a
        //nested number, if it's a `]` then we finished the number (so return it). Return from each
        //recursive call the size in chars to skip ahead
        Err("Didn't impl yet".to_string())
    }
}

#[aoc_generator(day18)]
fn day18_parse(input: &str) -> Result<Vec<SNumber>,String> {
    //What's the syntax to colllect into a result again?
    Ok(input.lines().map(|line| SNumber::from_str(line).unwrap() ).collect::<Vec<_>>())

}

#[aoc(day18, part1)]
pub fn day18_part1(snumbers: &Vec<SNumber> ) -> i64 {
    0
}

#[aoc(day18, part2)]
pub fn day18_part2(snumbers: &Vec<SNumber>) -> u128 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_log::test;

    #[test]
    fn day18_part1_small() {
        let inp = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

    }
}
