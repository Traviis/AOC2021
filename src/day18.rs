use std::str::FromStr;

#[derive(Debug,PartialEq,Eq)]
pub enum SNumber {
    Pair(Box<SNumber>,Box<SNumber>),
    Lit(i64)
}

/// Find substring that is enclosed by the brackets (but maintain internal brackets)
fn find_enclosing_brackets(val: &str) -> &str {
    println!("feb: {}", val);
    let mut chars = val.chars();
    assert_eq!('[',chars.next().unwrap());

    let mut depth = 1;
    let mut len = 1;

    for c in chars {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => (),
        };
        len += 1;
        if depth == 0 {
            break;
        }
    }

    &val[0..len]
}

impl FromStr for SNumber {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //Iterate over chars, if you see a `[` then it's the start of a new Snumber, if it's a
        //number literal, then it's part of the SNumber, if it's another `[` it's the start of a
        //nested number, if it's a `]` then we finished the number (so return it). Return from each
        //recursive call the size in chars to skip ahead
        Ok(SNumber::compose(line))

    }

//pub enum SNumber {
//    Pair(Box<SNumber>,Box<SNumber>),
//    Lit(u64)
//}

}

impl SNumber {

    fn len(self : &Self) -> usize {
        // [ + ] + , -- ? That factors in somehow
        match self {
            SNumber::Lit(lit) => lit.to_string().len(),
            SNumber::Pair(p1,p2) => p1.len() + p2.len(),
        }
    }

    fn compose(line: &str) -> Self {

        let enc_str = find_enclosing_brackets(line);
        println!("enc_str: {}", enc_str);
        let mut chars = enc_str.chars();
        assert_eq!('[', chars.next().unwrap());

        //let inp = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let c_next = chars.next().unwrap();

        let first_val : SNumber = if c_next == '[' {
            SNumber::compose(find_enclosing_brackets(&enc_str[1..enc_str.len()-1]))
        } else {
            //Is a number literal
            let val = String::from(c_next) + &chars.by_ref().take_while(|c| *c != ',' && *c != ']').collect::<String>();
            SNumber::Lit(val.parse::<i64>().unwrap())
        };

        println!("First_val {:?} size: {} chars: {:?}",first_val, first_val.len(), chars);
        assert_eq!(',', chars.next().unwrap());
        let mut chars = chars.by_ref().step_by(first_val.len());

        let c_next = chars.next().unwrap();
        println!("c_next {}", c_next);

        let second_val : SNumber = if c_next == '[' {
            SNumber::compose(find_enclosing_brackets(&enc_str[2+first_val.len()..]))
        } else {
            //Is a number literal
            let val = String::from(c_next) + &chars.by_ref().take_while(|c| *c != ',' && *c != ']').collect::<String>();
            SNumber::Lit(val.parse::<i64>().unwrap())
        };

        //SNumber::Pair(Box::new(first_val), Box::new(second_val))
        SNumber::Pair(Box::new(first_val), Box::new(second_val))
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
    fn day18_part1_parse_simple() {
        let inp = "[0,[8,7]]";
        let s_num = SNumber::from_str(inp);
        if let SNumber::Pair(p_1, p_2) = s_num.unwrap() {
            assert_eq!(*p_1, SNumber::Lit(0));
            assert_eq!(*p_2, SNumber::Pair( Box::new(SNumber::Lit(8)), Box::new(SNumber::Lit(7))));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn day18_part1_parse_long() {

        let inp = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        println!("{:?}", SNumber::from_str(inp));
    }

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
