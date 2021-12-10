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

//For 2, you have 1,3,4,7,8
// Looking for 5 as well

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

fn and_string(a: &str, b: &str) -> String {
    //Find only the unshared characters
    //unique_to_a = a.chars().iter().filter(|ach| bc.iter().contains(ach));
    let unique_a = a.chars().filter(|&ac| b.contains(ac)).collect::<String>();

    unique_a
}
fn xor_strings(a: &str, b: &str) -> String {
    //Find only the unshared characters
    //unique_to_a = a.chars().iter().filter(|ach| bc.iter().contains(ach));
    let unique_a = a.chars().filter(|&ac| !b.contains(ac)).collect::<String>();
    let unique_b = b.chars().filter(|&bc| !a.contains(bc)).collect::<String>();

    unique_a + &unique_b
}

fn sort_string_chars(st: &str) -> String {
    let mut chars = st.chars().collect::<Vec<_>>();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect::<String>()
}

#[aoc(day8, part2)]
pub fn day8_part2(input: &Vec<(Vec<String>, Vec<String>)>) -> u128 {
    //The above commented information encoded
    /*
    let number_to_segment_map = vec![
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]; //Index by the number you're curious about
    */

    let mut sum: u128 = 0;
    for (input, output) in input.iter() {
        // Each line has it's own translation
        //First determine which segments map to the easy numbers
        //Then taking those sets, of known values, look at the rest of the
        //You can determine which segment is mapped to which by knowing the sets that must be on,
        //and checking against the other ones you know

        let mut segment_to_number_map = input
            .iter()
            .map(|x| {
                (
                    match x.chars().count() {
                        //2 => number_to_segment_map[1],
                        //4 => number_to_segment_map[4],
                        //3 => number_to_segment_map[7],
                        //7 => number_to_segment_map[8],
                        2 => 1,
                        4 => 4,
                        3 => 7,
                        7 => 8,
                        _ => -1,
                    },
                    sort_string_chars(x),
                )
            })
            .filter(|(y, _)| *y != -1)
            .collect::<HashMap<i64, String>>();

        //If you have 7 and 1, the unique value from the string corresponds to a.
        // There has got to be a better way than just doing tricks one by one.
        let mut two_three_five = input
            .iter()
            .filter(|x| x.chars().count() == 5)
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut zero_six_nine = input
            .iter()
            .filter(|x| x.chars().count() == 6)
            .collect::<Vec<_>>();

        //Determine 3
        //If you take the set_difference between 3 and the known 1, the length will be 3.
        let three = two_three_five
            .iter()
            .filter(|candid| {
                //xor_strings(candid, number_to_segment_map[1])
                xor_strings(candid, segment_to_number_map.get(&1).unwrap())
                    .chars()
                    .count()
                    == 3
            })
            .fold("", |a, i| i);

        segment_to_number_map
            .entry(3)
            .or_insert(sort_string_chars(three));
        //segment_to_number_map
        //    .entry(sort_string_chars(three))
        //    .or_insert(3);

        two_three_five = two_three_five
            .iter()
            .filter(|x| x.to_string() != three)
            .map(|x| x.clone()) //I don't care anymore
            .collect::<Vec<_>>();
        //1,3,4,7,8
        //    0:      1:      2:      3:      4:
        //   aaaa    ....    aaaa    aaaa    ....
        //  b    c  .    c  .    c  .    c  b    c
        //  b    c  .    c  .    c  .    c  b    c
        //   ....    ....    dddd    dddd    dddd
        //  e    f  .    f  e    .  .    f  .    f
        //  e    f  .    f  e    .  .    f  .    f
        //   gggg    ....    gggg    gggg    ....
        //
        //    5:      6:      7:      8:      9:
        //   aaaa    aaaa    aaaa    aaaa    aaaa
        //  b    .  b    .  .    c  b    c  b    c
        //  b    .  b    .  .    c  b    c  b    c
        //   dddd    dddd    ....    dddd    dddd
        //  .    f  e    f  .    f  e    f  .    f
        //  .    f  e    f  .    f  e    f  .    f
        //   gggg    gggg    ....    gggg    gggg
        //
        //   4 & 2 = bcdf & acdeg = cd
        //   4 & 5 = bcdf & abdfg = bdf

        //Determine 5
        let five = two_three_five
            .iter()
            //.filter(|can| and_string(can, number_to_segment_map[4]).chars().count() == 3)
            .filter(|can| {
                and_string(can, segment_to_number_map.get(&4).unwrap())
                    .chars()
                    .count()
                    == 3
            })
            .fold("", |a, i| i);

        segment_to_number_map
            .entry(5)
            .or_insert(sort_string_chars(five));
        //segment_to_number_map
        //    .entry(sort_string_chars(two))
        //    .or_insert(2);

        two_three_five = two_three_five
            .iter()
            .filter(|x| x.to_string() != five)
            .map(|x| x.clone()) //I don't care anymore
            .collect::<Vec<_>>();

        //Determine 2
        let two = two_three_five[0].clone();
        segment_to_number_map
            .entry(2)
            .or_insert(sort_string_chars(&two));
        //  .entry(sort_string_chars(&five))
        //  .or_insert(5);

        // Determine 9
        let nine = zero_six_nine
            .iter()
            //.filter(|can| and_string(can, number_to_segment_map[3]).chars().count() == 5)
            .filter(|can| {
                and_string(can, segment_to_number_map.get(&3).unwrap())
                    .chars()
                    .count()
                    == 5
            })
            .fold("!!!!!!!!!!!!!!!", |a, i| i);

        zero_six_nine = zero_six_nine
            .iter()
            .filter(|x| x.to_string() != nine)
            .map(|x| x.clone()) //I don't care anymore
            .collect::<Vec<_>>();

        segment_to_number_map
            //    .entry(sort_string_chars(&nine))
            //    .or_insert(9);
            .entry(9)
            .or_insert(sort_string_chars(&nine));

        // Determine 0
        let zero = zero_six_nine
            .iter()
            //.filter(|can| and_string(can, number_to_segment_map[7]).chars().count() == 3)
            .filter(|can| {
                and_string(can, segment_to_number_map.get(&7).unwrap())
                    .chars()
                    .count()
                    == 3
            })
            .fold("", |a, i| i);

        zero_six_nine = zero_six_nine
            .iter()
            .filter(|x| x.to_string() != zero)
            .map(|x| x.clone()) //I don't care anymore
            .collect::<Vec<_>>();

        segment_to_number_map
            //    .entry(sort_string_chars(&nine))
            //    .or_insert(9);
            .entry(0)
            .or_insert(sort_string_chars(&zero));

        // Deduce 6
        let six = zero_six_nine[0].clone();
        segment_to_number_map
            // .entry(sort_string_chars(&six))
            // .or_insert(6);
            .entry(6)
            .or_insert(sort_string_chars(&six));

        println!("Trans Map: {:?}", segment_to_number_map);
        // Now translate
        let outval = output
            .iter()
            .map(|x| match_string(&segment_to_number_map, x).to_string())
            .collect::<String>();
        println!("Outval: {}", outval);
        sum += outval.parse::<u128>().unwrap();
    }

    sum
}

fn match_string(haystack: &HashMap<i64, String>, requested: &String) -> i64 {
    println!("{:?} Looking for {}", haystack, requested);
    let out_num = *haystack
        .iter()
        .find(|(_, v)| xor_strings(v, requested).chars().count() == 0)
        .unwrap()
        .0;
    println!("Looking for {} found {}", requested, out_num);
    out_num
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

    #[warn(dead_code)]
    fn get_short_test_input() -> &'static str {
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
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
    fn day8_part2_test_short() {
        //        let (input, output) = day8_parse(get_test_input());
        //        assert_eq!(day8_part1(&(input, output)), 26);
        assert_eq!(day8_part2(&day8_parse(get_short_test_input())), 5353);
    }
    #[test]
    fn day8_part1_test() {
        //        let (input, output) = day8_parse(get_test_input());
        //        assert_eq!(day8_part1(&(input, output)), 26);
        assert_eq!(day8_part1(&day8_parse(get_test_input())), 26);
    }
    #[test]
    fn day8_part2_test() {
        assert_eq!(day8_part2(&day8_parse(get_test_input())), 61229);
    }
}
