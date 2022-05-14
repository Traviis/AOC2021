use std::{str::FromStr, cmp::min};

#[derive(Debug,PartialEq,Eq,Clone)]
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
            SNumber::Pair(p1,p2) => p1.len() + p2.len() + 3, //Comma and brackets
        }
    }

    //This snailfish homework is about addition. To add two snailfish numbers, form a pair from the
    //left and right parameters of the addition operator. For example, [1,2] + [[3,4],5] becomes
    //[[1,2],[[3,4],5]].
    fn add(self: &Self, other: &Self) -> Self {
        //ugh
        SNumber::reduce(&SNumber::Pair(Box::new(self.clone()), Box::new(other.clone())))
    }

    fn reduce(self: &Self) -> Self {
        //To reduce a snailfish number, you must repeatedly do the first action in this list that
        //applies to the snailfish number: 
        //If any pair is nested inside four pairs, the leftmost such pair explodes.
        //If any regular number is 10 or greater, the leftmost such regular number splits.
        //Once no action in the above list applies, the snailfish number is reduced.
        //
        //During reduction, at most one action applies, after which the process returns to the top
        //of the list of actions. For example, if split produces a pair that meets the explode
        //criteria, that pair explodes before other splits occur.

        //To explode a pair, the pair's left value is added to the first regular number to the left of the
        //exploding pair (if any), and the pair's right value is added to the first regular number to the
        //right of the exploding pair (if any). Exploding pairs will always consist of two regular numbers.
        //Then, the entire exploding pair is replaced with the regular number 0.

        //Algo:
        // Find first pair that is nested within 4 from left to right (DFS search with left branch)
        // If found, explode, re-run this function on resultant number
        // if not found, find any number (left most) that is > 10 (DFS search that checks both
        // nodes but proceed always down left first) then split:
        //
        // To split a regular number, replace it with a pair; the left element of the pair should
        // be the regular number divided by two and rounded down, while the right element of the
        // pair should be the regular number divided by two and rounded up. For example, 10 becomes
        // [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
        //
        // if no action applied, the number is reduced
        
        // Need function that finds next Literal number to "the right" is this just DFS again 
        todo!()
    }

    //TODO: Needs tests
    fn split(&self) -> Self {
        if let SNumber::Lit(n) = self {
        return SNumber::Pair(
                Box::new(SNumber::Lit((*n as f64/2.0).floor() as i64)), 
                Box::new(SNumber::Lit((*n as f64/2.0).ceil() as i64))
                );
        } else {
            panic!("Invalid split on {:?}", self);
        }
    }

    fn compose(line: &str) -> Self {

        //let enc_str = find_enclosing_brackets(line);
        let enc_str = line;
        println!("enc_str: {}", enc_str);
        let mut chars = enc_str.chars();
        assert_eq!('[', chars.next().unwrap());

        //let inp = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let c_next = chars.next().unwrap();

        let first_val : SNumber = if c_next == '[' {
            SNumber::compose(find_enclosing_brackets(&enc_str[1..enc_str.len()-1]))
        } else {
            //Is a number literal
            let val = String::from(c_next) + &chars.by_ref().take_while(|c| *c != ',').collect::<String>();
            SNumber::Lit(val.parse::<i64>().unwrap())
        };

        println!("First_val {:?} size: {} chars: {:?}",first_val, first_val.len(), chars);
        for _ in 0..first_val.len()-1 {
            chars.next().unwrap();
        }
        //assert_eq!(',', chars.next().unwrap());

        println!("After stepping: {:?}", chars);

        let mut c_next = chars.next().unwrap();
        println!("c_next {}", c_next);

        //This is a disgusting hack, I need to rethink this
        if c_next == ',' {
            c_next = chars.next().unwrap();
        }

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

fn floor(n: i64) -> i64 {
    todo!()
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
    fn day18_part1_parse_simple2() {
        let inp = "[[1,2],3]";
        let s_num = SNumber::from_str(inp);
        if let SNumber::Pair(p_1, p_2) = s_num.unwrap() {
            assert_eq!(*p_1, SNumber::Pair( Box::new(SNumber::Lit(1)), Box::new(SNumber::Lit(2))));
            assert_eq!(*p_2, SNumber::Lit(3));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn day18_part1_parse_long() {

        let inp = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        println!("{:?}", SNumber::from_str(inp));
//Ok(Pair(Pair(Pair(Pair(Lit(1), Lit(2)), Pair(Lit(3), Lit(4))), Pair(Pair(Lit(5), Lit(6)), Pair(Lit(7), Lit(8)))), Lit(9))
        let inp2 = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        println!("{:?}", SNumber::from_str(inp2));
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
