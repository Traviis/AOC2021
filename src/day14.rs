use std::collections::HashMap;

type ConversionMap = HashMap<(char, char), char>;
type Polymer = HashMap<(char, char), u64>; //Map of pairs
type FirstLast = (char, char);

#[aoc_generator(day14)]
fn day14_parse(input: &str) -> (ConversionMap, Polymer, FirstLast) {
    let mut inp = input.split("\n\n");
    let mut polymer = Polymer::new();

    let chars = inp.clone().next().unwrap().chars().collect::<Vec<_>>();
    let first = *chars.first().unwrap();
    let last = *chars.last().unwrap();

    for slic in inp.next().unwrap().chars().collect::<Vec<_>>().windows(2) {
        //Push into polymer the start string
        *polymer.entry((slic[0], slic[1])).or_insert(0) += 1;
    }
    let c_map = inp
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (bef, aft) = line.split_once(" -> ").unwrap();
            let mut bchars = bef.chars();
            (
                (bchars.next().unwrap(), bchars.next().unwrap()),
                aft.chars().next().unwrap(),
            )
        })
        .collect::<ConversionMap>();

    (c_map, polymer, (first, last))
}

#[aoc(day14, part1)]
pub fn day14_part1(group: &(ConversionMap, Polymer, FirstLast)) -> u128 {
    poly_map(group, 10)
}

#[aoc(day14, part2)]
pub fn day14_part2(group: &(ConversionMap, Polymer, FirstLast)) -> u128 {
    poly_map(group, 40)
}

fn poly_map(
    (c_map, poly, (first, last)): &(ConversionMap, Polymer, FirstLast),
    steps: i32,
) -> u128 {
    let mut poly = poly.clone();

    for _ in 0..steps {
        let mut new_poly = Polymer::new();
        for ((a, c), n) in poly.iter() {
            //For every item in the polymer, update the occurance map
            let b = c_map[&(*a, *c)];
            *new_poly.entry((*a, b)).or_insert(0) += n;
            *new_poly.entry((b, *c)).or_insert(0) += n;
        }

        poly = new_poly;
        //println!("Poly {:?}", poly);
    }

    //Determine frequency
    //Everything is double counted due to overlap EXCEPT for the first and last char, so just
    //correct that after you get the frequency map
    let mut char_freq = HashMap::new();
    poly.iter()
        .flat_map(|((a, b), n)| [(*a, *n), (*b, *n)])
        .for_each(|(c, n)| {
            *char_freq.entry(c).or_insert(0) += n;
        });

    *char_freq.get_mut(first).unwrap() += 1;
    *char_freq.get_mut(last).unwrap() += 1;

    //Correct for duplication
    //println!("Dub FreqMap: {:?}", char_freq);
    char_freq = char_freq.iter().map(|(&c, &n)| (c, n / 2)).collect();

    let (_most_common, most_val) =
        char_freq.iter().fold(
            ('!', 0),
            |(a, cm), (&c, &n)| if n > cm { (c, n) } else { (a, cm) },
        );

    let (_least_common, least_val) =
        char_freq.iter().fold(
            ('!', u64::MAX),
            |(a, cm), (&c, &n)| if n < cm { (c, n) } else { (a, cm) },
        );
    /*
    println!(
        "Most common: {} ({}), Least Common: {} ({})",
        most_common, most_val, least_common, least_val
    );
    */
    (most_val - least_val) as u128
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    }

    #[test]
    fn day14_part1() {
        assert_eq!(super::day14_part1(&day14_parse(get_test_input())), 1588);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(
            super::day14_part2(&day14_parse(get_test_input())),
            2188189693529
        );
    }
}
