use std::cmp;

#[aoc_generator(day7)]
fn day7_parse(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(str::trim)
        //flat_map is a cute way to throw out the Err results.
        .flat_map(str::parse::<i64>)
        .collect::<Vec<i64>>()
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<i64>) -> u128 {
    let avg = input.iter().sum::<i64>() / input.iter().count() as i64;
    //Average is the common line
    let mut min = i64::MAX;
    //I don't really have mathmatical proof that no one crab will have to move no more than
    //average to find a solution, but it just felt like the intuitive thing to do (and it passed
    //the test cases).
    //
    //Taken to an extreme, if you had 100 crabs at 1 and 1 crab at 20, the average would be
    //~1.1881, (see test), and the required moves would be 19 because all of the 1 crabs sit
    //and do nothing, and the 20 crab move 19 spaces to be close to 1. I still can't derive
    //proof here of why you need only check the average distance for the set.
    //
    //I think you can also start at the minimum from the input set, since later when we do (x-i).abs()
    //the minumum it can be is 0 (I required no movement). This probably also explains why you need
    //only check the ceil(avg) inclusive. If the average line is 1.118 as in my above statement
    //+1 in case the average is non integral and truncates. Likewise, the maximum value that can
    //appear there is if you take the position of any one crab, and subtract the average, in this
    //case, 19 (flooring it). Thus, there is no way to ever exceed that value due to the abs(). I'm
    //sure there is a fancy math way to write this out, but I don't know it.
    //
    //If I went up to the maximum of say, 20 instead of the average I would see this equation: (20
    //- 20).abs() which equals 0 Which is not useful, (especially since I already have it). Any
    //number > than the max is further wasteful, since if you're trying to converage to the average
    //line, you need not ever exceed above that difference.
    let min_input = input.iter().min().unwrap();
    for i in *min_input..avg + 2 {
        //Linear loss, Just sum every step; The best solution will be found by having every crab
        //move AT MOST the average distance. You could really just brute force this, there are not
        //that many numbers, but this helps out a bit.
        let possible_set = input.iter().map(|x| (x - i).abs()).sum::<i64>();
        //println!("Pos set {}", possible_set);
        min = cmp::min(possible_set, min);
    }
    min as u128
}
#[aoc(day7, part2)]
pub fn part2(input: &Vec<i64>) -> u128 {
    let avg = input.iter().sum::<i64>() as f64 / input.iter().count() as f64;
    //Average is the common line
    let mut min = i64::MAX;
    // Derp, floats average can be you know non integral, can either +2 it to always be safe, or,
    // just ceiling it which does the same thing since integral division will truncate
    let min_input = input.iter().min().unwrap();
    for i in *min_input..avg.ceil() as i64 + 1 {
        let possible_set = input
            .iter()
            .map(|x| {
                //Non-linear loss
                //Rounding issue for non even numbers?
                // 1 => 1
                // 2 => 3 (1 + 2)
                // 3 => 6 (1 + 2 + 3)
                // 4 => 10 (1 + 2 +3 +4)
                //
                //Factorial but with addition.... Something to do with binomials..? Googling
                //Factorial with addition gives this:
                // https://math.stackexchange.com/questions/593318/factorial-but-with-addition/593323
                let ab = (x - i).abs();
                //I'll use this one, since it's the one I actually understand how it works and is
                //derived, but I'll test to ensure the other solutions work.
                (1..ab + 1).into_iter().sum::<i64>() as i64
            })
            .sum::<i64>() as i64;
        //println!("Pos set {}", possible_set);
        min = cmp::min(possible_set, min);
    }
    min as u128
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn day7_part1() {
        assert_eq!(part1(&day7_parse(get_test_input())), 37);
    }
    #[test]
    fn day7_part2() {
        //Should be 168?
        assert_eq!(part2(&day7_parse(get_test_input())), 168);
    }

    #[test]
    fn sigma_test() {
        for n in 0..100 {
            let manual_sig = (1..n + 1).into_iter().sum::<i64>() as i64;
            assert_eq!(manual_sig, (n.pow(2) + n) / 2);
            assert_eq!(manual_sig, (n * (n + 1)) / 2);
            //All of these equations appear to be equal
        }
    }

    #[test]
    fn test_average_intuition() {
        let mut insane_case = vec![];
        for _ in 0..100 {
            insane_case.push(1);
        }
        insane_case.push(20);
        assert_eq!(part1(&insane_case), 19);
        println!("Required linear moves {}", part1(&insane_case));
    }
}
