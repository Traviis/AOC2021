use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    FoldX(i32),
    FoldY(i32),
}

#[aoc_generator(day13)]
fn day13_parse(input: &str) -> (HashSet<(i32, i32)>, Vec<Instruction>) {
    let mut inp = input.split("\n\n");
    let dots = inp
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut spl = line.split(",").flat_map(str::parse::<i32>);
            (spl.next().unwrap(), spl.next().unwrap())
        })
        .collect::<HashSet<(i32, i32)>>();

    let inst = inp
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let v = line.split(" ").last().unwrap();
            let mut axis_set = v.split("=");
            let axis = axis_set.next().unwrap();
            let val = axis_set.next().unwrap().parse::<i32>().unwrap();
            match axis {
                "y" => Instruction::FoldY(val),
                "x" => Instruction::FoldX(val),
                _ => panic!("Bad axis"),
            }
        })
        .collect::<Vec<_>>();

    (dots, inst)
}

#[aoc(day13, part1)]
pub fn day13_part1() -> u128 {
    println!("{:?} => {:?}", 
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
