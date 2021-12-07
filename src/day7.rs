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
    let min_input = input.iter().min().unwrap();
    let mut min = i64::MAX;
    for i in *min_input..avg + 1 {
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
    let min_input = input.iter().min().unwrap();
    let mut min = i64::MAX;
    // Derp, floats average can be you know non integral, can either +2 it to always be safe, or,
    // just ceiling it which does the same thing since integral division will truncate
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
}
