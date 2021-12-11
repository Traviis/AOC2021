use std::collections::HashSet;

#[aoc_generator(day11)]
fn day11_parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string())
                .flat_map(|x| str::parse::<i32>(&x))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<_>>>()
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<i32>>) {
    for y in 0..map.iter().count() {
        for x in 0..map[0].iter().count() {
            //print!("({},{}) => {}", x, y, map[y][x]);
            print!("{}", map[y][x]);
        }
        print!("\n");
    }
}

fn prop(map: &mut Vec<Vec<i32>>, flash_set: &mut HashSet<(i32, i32)>, x: i32, y: i32) -> i32 {
    let frames = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut flash_count = 0;
    let max_x = map[0].iter().count() as i32;
    let max_y = map.iter().count() as i32;

    for frame in frames.iter() {
        let cur = (x + frame.0, y + frame.1);
        if let Some(_) = flash_set.get(&cur) {
            //If we already flashed this step don't do it here
            continue;
        }
        if cur.0 < 0 || cur.0 >= max_x || cur.1 < 0 || cur.1 >= max_y {
            //Out of bounds ignore
            continue;
        }
        let refe = &mut map[cur.1 as usize][cur.0 as usize];
        *refe += 1;
        if *refe > 9 {
            flash_set.insert((cur.0, cur.1));
            flash_count += 1;
            flash_count += prop(map, flash_set, cur.0, cur.1);
        }
    }
    flash_count
}

#[aoc(day11, part1)]
pub fn day11_part1(map: &Vec<Vec<i32>>) -> u128 {
    flasher(100, false, map)
}
#[aoc(day11, part2)]
pub fn day11_part2(map: &Vec<Vec<i32>>) -> u128 {
    //Just keep going until we flash
    // Assume it will be less than 9999999 steps...
    flasher(99999999, true, map)
}

fn flasher(steps: i32, find_sync: bool, map: &Vec<Vec<i32>>) -> u128 {
    //print_map(&map);
    let mut map = map.clone();
    let mut flashes = 0;

    let octo_count = map.iter().count() * map[0].iter().count();

    for step in 0..steps {
        let mut flash_set: HashSet<(i32, i32)> = HashSet::new(); //Set of those that have flashed

        //All octopi increase their energy by 1
        for y in 0..map.iter().count() {
            for x in 0..map[0].iter().count() {
                map[y][x] += 1;
            }
        }

        //Flash all 10s (GREATER THAN 9; sneaky; I read that wrong and assumed it meant when it
        //HITS 9)
        for y in 0..map.iter().count() {
            for x in 0..map[0].iter().count() {
                if let Some(_) = flash_set.get(&(x as i32, y as i32)) {
                    continue; //Already flashed
                }
                if map[y][x] > 9 {
                    flash_set.insert((x as i32, y as i32));
                    flashes += 1;
                    flashes += prop(&mut map, &mut flash_set, x as i32, y as i32);
                }
            }
        }

        for (x, y) in flash_set.iter() {
            map[*y as usize][*x as usize] = 0;
        }

        if flash_set.iter().count() == octo_count && find_sync {
            return (step + 1) as u128;
        }

        //println!("After Step {}", step + 1);
        //print_map(&map);
    }
    flashes as u128
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }
    fn get_short_test_input() -> &'static str {
        "11111
19991
19191
19991
11111"
    }
    #[test]
    fn day11_part1_short() {
        assert_eq!(
            super::flasher(2, false, &day11_parse(get_short_test_input())),
            9
        );
    }

    #[test]
    fn day11_part1_medium() {
        assert_eq!(
            super::flasher(10, false, &day11_parse(get_test_input())),
            204
        );
    }

    #[test]
    fn day11_part1() {
        assert_eq!(super::day11_part1(&day11_parse(get_test_input())), 1656);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(super::day11_part2(&day11_parse(get_test_input())), 195);
    }
}
