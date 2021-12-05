use std::cmp;
use std::collections::HashMap;

//Returning a vector is wasteful when we could pass in a mutable reference to the hashmap and
//populate it directly, however, let's leave this how it is it's easier to unit test (if there were
//unit tests) and doesn't mix the logic around for a probably minor speed increase.
fn interpolate(consider_diags: bool, x1: u64, y1: u64, x2: u64, y2: u64) -> Vec<(u64, u64)> {
    let mut output = vec![];

    if (x1 != x2 && y1 != y2) && !consider_diags {
        return output; //Ignore diags
    }

    if x1 == x2 {
        //Vertical
        for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
            output.push((x1, y));
        }
    } else if y1 == y2 {
        //Horizontal
        for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
            output.push((x, y1));
        }
    } else {
        // X Must increase, becuase of using range in rust doesn't work in reverse.
        // You could use the rev() on the range, however, since the values are tied to another,
        // that seems confusing.
        let (left_most, right_most) = if x1 < x2 {
            ((x1, y1), (x2, y2))
        } else {
            ((x2, y2), (x1, y1))
        };
        let slope = if left_most.1 < right_most.1 {
            1 as i64
        } else {
            -1 as i64
        };
        // y = mx + b   m == [-1,1]
        // b = y - mx
        let b = (left_most.1 as i64) - slope * (left_most.0 as i64);
        for x in left_most.0..right_most.0 + 1 {
            output.push((x, (slope * (x as i64) + b) as u64));
        }

        /*
             //Rust doesn't like negative ranges ....
            let x_range = x1..x2+1;
            let y_range = y1..y2+1;
            let mut line = (x_range)
                .into_iter()
                .zip((y_range).into_iter())
                .collect::<Vec<_>>();
            output.append(&mut line);
        }
            */
    }

    //println!("Line{:?}", output);

    output
}

//Surprisingly, this had no real extra effect disproving my above statement
fn interpolate_direct(
    consider_diags: bool,
    h_map: &mut HashMap<(u64, u64), u64>,
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
) {
    if (x1 != x2 && y1 != y2) && !consider_diags {
        return; //Ignore diags
    }

    if x1 == x2 {
        //Vertical
        for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
            let point = h_map.entry((x1, y)).or_insert(0);
            *point += 1;
        }
    } else if y1 == y2 {
        //Horizontal
        for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
            let point = h_map.entry((x, y1)).or_insert(0);
            *point += 1;
        }
    } else {
        // X Must increase, becuase of using range in rust doesn't work in reverse.
        // You could use the rev() on the range, however, since the values are tied to another,
        // that seems confusing.
        let (left_most, right_most) = if x1 < x2 {
            ((x1, y1), (x2, y2))
        } else {
            ((x2, y2), (x1, y1))
        };
        let slope = if left_most.1 < right_most.1 {
            1 as i64
        } else {
            -1 as i64
        };
        // y = mx + b   m == [-1,1]
        // b = y - mx
        let b = (left_most.1 as i64) - slope * (left_most.0 as i64);
        for x in left_most.0..right_most.0 + 1 {
            let in_point = (x, (slope * (x as i64) + b) as u64);
            let point = h_map.entry(in_point).or_insert(0);
            *point += 1;
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(u64, u64), u64>) {
    let (max_x, max_y) = map.iter().fold((0, 0), |(mx, ym), ((x, y), _)| {
        (if *x > mx { *x } else { mx }, if *y > ym { *y } else { ym })
    });

    //println!("Max X {} Max Y {}", max_x, max_y);
    for y in 0..max_y + 1 {
        let mut line = String::new();
        for x in 0..max_x + 1 {
            match map.get(&(x, y)) {
                Some(v) => line.push_str(&v.to_string()[..]),
                None => line.push('.'),
            }
        }
        println!("{}", line);
    }
}

#[aoc_generator(day5, part1, vecreturn)]
fn part1_parse_vec(input: &str) -> HashMap<(u64, u64), u64> {
    parse_input(false, false, input)
}

#[aoc_generator(day5, part2, vecreturn)]
fn part2_parse_vec(input: &str) -> HashMap<(u64, u64), u64> {
    parse_input(true, false, input)
}
#[aoc_generator(day5, part1, direct)]
fn part1_parse_direct(input: &str) -> HashMap<(u64, u64), u64> {
    parse_input(false, true, input)
}

#[aoc_generator(day5, part2, direct)]
fn part2_parse_direct(input: &str) -> HashMap<(u64, u64), u64> {
    parse_input(true, true, input)
}

fn parse_input(consider_diags: bool, direct: bool, input: &str) -> HashMap<(u64, u64), u64> {
    let mut map = HashMap::new();
    input.split("\n").map(str::trim).for_each(|line| {
        let sp = line
            .split(&[',', ' '][..]) //Lame, Rust wants a slice
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<u64>>();
        //assert!(sp.iter().count() == 4);
        //println!(" ({},{}) => ({},{})", sp[0], sp[1], sp[2], sp[3]);
        if direct {
            interpolate_direct(consider_diags, &mut map, sp[0], sp[1], sp[2], sp[3]);
        } else {
            for point in interpolate(consider_diags, sp[0], sp[1], sp[2], sp[3]) {
                //   println!("{:?}", point);
                let point = map.entry(point).or_insert(0);
                *point += 1;
            }
        }
    });

    map
}

#[aoc(day5, part2, vecreturn)]
pub fn part2_vec(map: &HashMap<(u64, u64), u64>) -> u64 {
    map.iter().filter(|(_, &v)| v > 1).count() as u64
}
#[aoc(day5, part2, direct)]
pub fn part2_direct(map: &HashMap<(u64, u64), u64>) -> u64 {
    map.iter().filter(|(_, &v)| v > 1).count() as u64
}

#[aoc(day5, part1, vecreturn)]
pub fn part1_vec(map: &HashMap<(u64, u64), u64>) -> u64 {
    map.iter().filter(|(_, &v)| v > 1).count() as u64
}

#[aoc(day5, part1, direct)]
pub fn part1_direct(map: &HashMap<(u64, u64), u64>) -> u64 {
    map.iter().filter(|(_, &v)| v > 1).count() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
    }

    #[test]
    fn example_part1_vec() {
        assert_eq!(part1_vec(&part1_parse_vec(get_test_input())), 5);
    }

    #[test]
    fn example_part1_direct() {
        assert_eq!(part1_direct(&part1_parse_direct(get_test_input())), 5);
    }

    #[test]
    fn example_part2_vec() {
        assert_eq!(part2_vec(&part2_parse_vec(get_test_input())), 12);
    }

    #[test]
    fn example_part2_direct() {
        assert_eq!(part2_direct(&part2_parse_direct(get_test_input())), 12);
    }
}
