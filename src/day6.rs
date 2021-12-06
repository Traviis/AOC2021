use std::collections::HashMap;

#[aoc_generator(day6)]
fn day6_parse(input: &str) -> HashMap<i64, u64> {
    let mut starting_map: HashMap<i64, u64> = HashMap::new();
    input.split(",").map(str::parse::<i64>).for_each(|val| {
        let entr = starting_map.entry(val.unwrap()).or_insert(0);
        *entr += 1;
    });

    starting_map
}

#[aoc(day6, part1)]
pub fn part1(umap: &HashMap<i64, u64>) -> u64 {
    day_param(80, umap)
}
#[aoc(day6, part2)]
pub fn part2(umap: &HashMap<i64, u64>) -> u64 {
    day_param(256, umap)
}

pub fn day_param(sim_days: i64, umap: &HashMap<i64, u64>) -> u64 {
    let mut map = umap.clone();
    #[allow(unused_variables)]
    for day in 0..sim_days {
        //For every set, move it "down" one day.
        // If day 5s have 10, set day 4 to 10
        /*
        println!(
            "Start Day {} Count {}",
            day,
            map.iter().fold(0, |a, (_, v)| a + v)
        );
        */
        map.iter().fold(0, |a, (_, v)| {
            (a as u64)
                .checked_add(*v)
                .unwrap_or_else(|| panic!("Overflowed on day {}", day))
        }) as i64;

        //Decrement
        let mut last_count = 0;
        for group in (-1..9).rev() {
            let ent = map.entry(group).or_insert(0);
            let cur_count = ent.clone();
            //println!("{} => {}", group, cur_count);

            *ent = last_count;
            last_count = cur_count;
        }

        //If the fish is at 0, then it will create a new 8, and it will reset itself to a 6

        //Spawn and reset
        let zero_set = map.entry(-1).or_insert(0); //0s just got decremented, so look for -1
        let zero_count = *zero_set;
        *zero_set = 0;

        let six_set = map.entry(6).or_insert(0); //Find the new 6 group and add to it
        *six_set += zero_count as u64;

        let eight_set = map.entry(8).or_insert(0); //find the new 8 group and add to it
        *eight_set += zero_count as u64;
    }

    map.iter().fold(0, |a, (_, v)| a + v)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "3,4,3,1,2"
    }

    #[test]
    fn day6_part1() {
        assert_eq!(part1(&day6_parse(get_test_input())), 5934);
    }
    #[test]
    fn day6_part2() {
        assert_eq!(part2(&day6_parse(get_test_input())), 26984457539);
    }
    #[test]
    //#[should_panic]
    //And just because I was curious, how fast would this exponentially run out of space, the
    //answer (for this starting set) is 482 days when it would overrun its i64.
    //If I use a u64 instead, it's only makes it to 490 days
    fn day6_rollover_detecter() {
        day_param(99999999999999999, &day6_parse(get_test_input()));
    }
}
