use std::collections::HashSet;

#[derive(Debug)]
pub enum Instruction {
    FoldX(i32),
    FoldY(i32),
}

fn print_paper(hash_set: &HashSet<(i32, i32)>) {
    let max_y = hash_set.iter().map(|(_, y)| y).max().unwrap();
    let max_x = hash_set.iter().map(|(x, _)| x).max().unwrap();
    for y in 0..*max_y + 1 {
        for x in 0..*max_x + 1 {
            let v = match hash_set.get(&(x, y)) {
                Some(_) => '#',
                None => '.',
            };
            print!("{}", v);
        }
        print!("\n");
    }
    println!("XXXXXXXXXXXXXXXXXX");
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
pub fn day13_part1(group: &(HashSet<(i32, i32)>, Vec<Instruction>)) -> u128 {
    folder(group, 1)
}

pub fn folder(
    (coordinates, instructions): &(HashSet<(i32, i32)>, Vec<Instruction>),
    how_many_folds: i32,
) -> u128 {
    //println!("{:?} => {:?}", coordinates, instructions);
    //print_paper(coordinates);
    //Assume the following, fold lines have no dots, there will always be an equal number of
    //lines/columns on each side (thus always odd), fold will always be in the center (via
    //inference from always equal)
    let mut current_paper = coordinates.clone();
    for (idx, inst) in instructions.iter().enumerate() {
        //println!("Folding {:?}", inst);
        match inst {
            Instruction::FoldY(axis) => {
                //Find all the points that need to be reflected over
                // Clone, because we can't iterate over something and remove from it at the same
                // time
                let points_to_reflect = current_paper
                    .clone()
                    .into_iter()
                    .filter(|(_, y)| y > axis)
                    .collect::<Vec<_>>();
                for &(x, y) in points_to_reflect.iter() {
                    //Remove it from paper
                    current_paper.remove(&(x, y));
                    // Reflect it over the axis
                    current_paper.insert((x, axis - (y - axis)));
                }
            }
            Instruction::FoldX(axis) => {
                let points_to_reflect = current_paper
                    .clone()
                    .into_iter()
                    .filter(|(x, _)| x > axis)
                    .collect::<Vec<_>>();
                for &(x, y) in points_to_reflect.iter() {
                    //Remove it from paper
                    current_paper.remove(&(x, y));
                    // Reflect it over the axis
                    current_paper.insert((axis - (x - axis), y));
                }
            }
        }
        //Stop early
        if idx + 1 == how_many_folds as usize {
            break;
        }
    }

    //Cheezy way of saying: "Don't print part 1"
    if how_many_folds > 1 {
        print_paper(&current_paper);
    }

    current_paper.iter().count() as u128
}
#[aoc(day13, part2)]
pub fn day13_part2(group: &(HashSet<(i32, i32)>, Vec<Instruction>)) -> u128 {
    //println!("{:?} => {:?}", coordinates, instructions);
    folder(group, group.1.iter().count() as i32)
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

fold along y=7
fold along x=5"
    }

    #[test]
    fn day13_part1() {
        assert_eq!(super::day13_part1(&day13_parse(get_test_input())), 17);
    }

    #[test]
    fn day13_part2() {
        //You can't really test this.... It's visual, and decoding letters would take FOREVER
        //assert_eq!(super::day13_part2(&day13_parse(get_test_input())), 36);
    }
}
