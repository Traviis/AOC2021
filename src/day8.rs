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
        //println!("Input {:?}", input);

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
                //println!("Out: {} chars {}, mat {}", st, st.chars().count(), mat);
                mat
            })
            .count();
        //println!("{:?} line out_count {}", output, out_count);
        sum += out_count as u128;
    }

    sum
}

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
    a.chars().filter(|&ac| b.contains(ac)).collect::<String>()
}
fn xor_strings(a: &str, b: &str) -> String {
    let unique_a = a.chars().filter(|&ac| !b.contains(ac)).collect::<String>();
    let unique_b = b.chars().filter(|&bc| !a.contains(bc)).collect::<String>();

    unique_a + &unique_b
}

fn sort_string_chars(st: &str) -> String {
    let mut chars = st.chars().collect::<Vec<_>>();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect::<String>()
}
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

#[aoc(day8, part2)]
pub fn day8_part2(input: &Vec<(Vec<String>, Vec<String>)>) -> u128 {
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

        //Grab the two groups of numbers you must puzzle out
        let mut two_three_five = input
            .iter()
            .filter(|x| x.chars().count() == 5)
            .collect::<Vec<_>>();

        let mut zero_six_nine = input
            .iter()
            .filter(|x| x.chars().count() == 6)
            .collect::<Vec<_>>();

        //Determine 3
        let three = two_three_five
            .iter()
            .find(|candid| {
                xor_strings(candid, segment_to_number_map.get(&1).unwrap())
                    .chars()
                    .count()
                    == 3
            })
            .unwrap()
            .clone();

        segment_to_number_map.insert(3, sort_string_chars(three));
        two_three_five.retain(|x| *x != three);

        //Determine 5
        let five = two_three_five
            .iter()
            .find(|can| {
                and_string(can, segment_to_number_map.get(&4).unwrap())
                    .chars()
                    .count()
                    == 3
            })
            .unwrap()
            .clone();

        segment_to_number_map.insert(5, sort_string_chars(five));
        two_three_five.retain(|x| *x != five);

        //Determine 2
        segment_to_number_map.insert(2, sort_string_chars(two_three_five[0]));

        // Determine 9
        let nine = zero_six_nine
            .iter()
            .find(|can| {
                and_string(can, segment_to_number_map.get(&3).unwrap())
                    .chars()
                    .count()
                    == 5
            })
            .unwrap()
            .clone();

        zero_six_nine.retain(|x| *x != nine);
        segment_to_number_map.insert(9, sort_string_chars(nine));

        // Determine 0
        let zero = zero_six_nine
            .iter()
            .find(|can| {
                and_string(can, segment_to_number_map.get(&7).unwrap())
                    .chars()
                    .count()
                    == 3
            })
            .unwrap()
            .clone();

        zero_six_nine.retain(|x| *x != zero);
        segment_to_number_map.insert(0, sort_string_chars(zero));

        // Deduce 6
        let six = zero_six_nine[0].clone();
        segment_to_number_map.insert(6, sort_string_chars(&six));

        //println!("Trans Map: {:?}", segment_to_number_map);
        // Now translate
        let outval = output
            .iter()
            .map(|x| match_string(&segment_to_number_map, x).to_string())
            .collect::<String>();
        //println!("Outval: {}", outval);
        sum += outval.parse::<u128>().unwrap();
    }

    sum
}

fn match_string(haystack: &HashMap<i64, String>, requested: &String) -> i64 {
    let out_num = *haystack
        .iter()
        .find(|(_, v)| xor_strings(v, requested).chars().count() == 0)
        .unwrap()
        .0;
    out_num
}

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
        assert_eq!(day8_part2(&day8_parse(get_short_test_input())), 5353);
    }
    #[test]
    fn day8_part1_test() {
        assert_eq!(day8_part1(&day8_parse(get_test_input())), 26);
    }
    #[test]
    fn day8_part2_test() {
        assert_eq!(day8_part2(&day8_parse(get_test_input())), 61229);
    }
}
