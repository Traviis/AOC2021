use std::collections::HashMap;

#[aoc_generator(day8)]
fn day8_parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let lines = input.lines();
    let mut output: Vec<(Vec<String>, Vec<String>)> = vec![];

    for line in lines {
        let mut segments = line.split("|");
        let input = segments
            .next()
            .unwrap()
            .split(" ")
            .map(str::trim)
            .map(|x| x.to_string())
            .filter(|x| x.chars().count() > 0)
            .collect::<Vec<String>>();
        println!("Input {:?}", input);

        let loutput = segments
            .next()
            .unwrap()
            .split(" ")
            .map(str::trim)
            .map(|x| x.to_string())
            .filter(|x| x.chars().count() > 0)
            .collect::<Vec<String>>();

        output.push((input, loutput));
    }

    output
}

#[aoc(day8, part1)]
pub fn day8_part1(input: &Vec<(Vec<String>, Vec<String>)>) -> u128 {
    let mut sum: u128 = 0;
    for (_, output) in input.iter() {
        let out_count = output
            .iter()
            .filter(|st| {
                let mat = match st.chars().count() {
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                };
                println!("Out: {} chars {}, mat {}", st, st.chars().count(), mat);
                mat
            })
            .count();
        println!("{:?} line out_count {}", output, out_count);
        sum += out_count as u128;
    }

    sum
}
//  0:      1:      2:      3:      4:
// aaaa    ....    aaaa    aaaa    ....
//b    c  .    c  .    c  .    c  b    c
//b    c  .    c  .    c  .    c  b    c
// ....    ....    dddd    dddd    dddd
//e    f  .    f  e    .  .    f  .    f
//e    f  .    f  e    .  .    f  .    f
// gggg    ....    gggg    gggg    ....
//
//  5:      6:      7:      8:      9:
// aaaa    aaaa    aaaa    aaaa    aaaa
//b    .  b    .  .    c  b    c  b    c
//b    .  b    .  .    c  b    c  b    c
// dddd    dddd    ....    dddd    dddd
//.    f  e    f  .    f  e    f  .    f
//.    f  e    f  .    f  e    f  .    f
// gggg    gggg    ....    gggg    gggg
//
//
// Char => Number of segments
// 0 => 6
// 1 => 2 //unique
// 2 => 5
// 3 => 5
// 4 => 4 //unique
// 5 => 5
// 6 => 6
// 7 => 3 //Unique
// 8 => 7 //unique
// 9 => 6
// If 6 segments, it's either 9, 6, or 0
// if 5 segments, it's either 5, 3, or 2
//
fn xor_strings(a: &str, b: &str) -> String {
    //Find only the unshared characters
    let ac = a.chars().collect::<Vec<_>>();
    let bc = b.chars().collect::<Vec<_>>();
    //unique_to_a = a.chars().iter().filter(|ach| bc.iter().contains(ach));
    let unique_a = a.chars().filter(|&ac| !b.contains(ac)).collect::<String>();
    let unique_b = b.chars().filter(|&bc| !a.contains(bc)).collect::<String>();

    unique_a + &unique_b
}

#[aoc(day8, part2)]
pub fn day8_part2(input: &Vec<(Vec<String>, Vec<String>)>) -> u128 {
    //The above commented information encoded
    let number_to_segment_map = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]; //Index by the number you're curious about

    let mut sum: u128 = 0;
    for (input, output) in input.iter() {
        // Each line has it's own translation
        //First determine which segments map to the easy numbers
        //Then taking those sets, of known values, look at the rest of the
        //You can determine which segment is mapped to which by knowing the sets that must be on,
        //and checking against the other ones you know

        let segment_to_number_map = input
            .iter()
            .map(|x| {
                (
                    x.clone(),
                    match x.chars().count() {
                        //2 => number_to_segment_map[1],
                        //4 => number_to_segment_map[4],
                        //3 => number_to_segment_map[7],
                        //7 => number_to_segment_map[8],
                        2 => 1,
                        4 => 4,
                        3 => 7,
                        7 => 8,
                        _ => 0,
                    },
                )
            })
            .filter(|(_, y)| *y != 0)
            .collect::<HashMap<String, i64>>();

        //If you have 7 and 1, the unique value from the string corresponds to a.
        // There has got to be a better way than just doing tricks one by one.
        let two_three_five = input
            .iter()
            .filter(|x| x.chars().count() == 5)
            .collect::<Vec<_>>();
        let zero_six_nine = input
            .iter()
            .filter(|x| x.chars().count() == 6)
            .collect::<Vec<_>>();

        //Determine 3
        //If you take the set_difference between 3 and the known 1, the length will be 3.
        let three = two_three_five
            .iter()
            .filter(|candid| {
                xor_strings(candid, number_to_segment_map[1])
                    .chars()
                    .count()
                    == 3
            })
            .fold("", |a, i| i);
        segment_to_number_map.entry(&three.to_string()).or_insert(3);

        //Determine 2
        //Determine 5
    }

    0
}
/*
#[aoc(day8, part2)]
pub fn day8_part2(input: (&Vec<&str>, &Vec<&str>)) -> u128 {
    0
}
*/

#[cfg(test)]
mod tests {

    use super::*;

    fn get_short_test_input() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
    }
    fn get_test_input() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    #[test]
    fn day8_part1_test() {
        //        let (input, output) = day8_parse(get_test_input());
        //        assert_eq!(day8_part1(&(input, output)), 26);
        assert_eq!(day8_part1(&day8_parse(get_test_input())), 26);
    }
    #[test]
    fn day8_part2_test() {
        //Should be 168?
        //assert_eq!(part2(&day8_parse(get_test_input())), 168);
    }
}
