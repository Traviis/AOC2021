use std::collections::HashSet;
use std::fmt;

pub struct Map {
    map: Vec<Vec<i64>>,
    max_x: i64, //i64 is overkill, but whatever
    max_y: i64,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|row| {
                row.chars()
                    .map(|x| x.to_string())
                    .flat_map(|x| str::parse::<i64>(&x))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let max_x = map[0].iter().count() as i64;
        let max_y = map.iter().count() as i64;

        Map { map, max_x, max_y }
    }

    fn find_basin(&self, basin: &mut HashSet<(i64, i64)>, x: i64, y: i64) {
        //Find left
        let mut to_search = HashSet::new(); //Track how many are added, then return that, if this is 0, stop considering
        'left: for cx in (0..x).rev() {
            if let Some(v) = self.get(cx, y) {
                if v != 9 {
                    if let None = basin.get(&(cx, y)) {
                        basin.insert((cx, y));
                        to_search.insert((cx, y));
                    }
                } else {
                    break 'left;
                }
            } else {
                break 'left;
            }
        }
        //Find right
        'right: for cx in x + 1..self.max_x + 1 {
            if let Some(v) = self.get(cx, y) {
                if v != 9 {
                    if let None = basin.get(&(cx, y)) {
                        basin.insert((cx, y));
                        to_search.insert((cx, y));
                    }
                } else {
                    break 'right;
                }
            } else {
                break 'right;
            }
        }
        //Find up
        'up: for cy in y + 1..self.max_y + 1 {
            if let Some(v) = self.get(x, cy) {
                if v != 9 {
                    if let None = basin.get(&(x, cy)) {
                        basin.insert((x, cy));
                        to_search.insert((x, cy));
                    }
                } else {
                    break 'up;
                }
            } else {
                break 'up;
            }
        }
        //Find Down
        'down: for cy in (0..y).rev() {
            if let Some(v) = self.get(x, cy) {
                if v != 9 {
                    if let None = basin.get(&(x, cy)) {
                        basin.insert((x, cy));
                        to_search.insert((x, cy));
                    }
                } else {
                    break 'down;
                }
            } else {
                break 'down;
            }
        }
        //End cardinal
        //Recurse!
        for (x, y) in to_search.into_iter() {
            self.find_basin(basin, x, y);
        }
    }

    fn find_basins(&self) -> Vec<Vec<(i64, i64)>> {
        let low_points = self.find_low_points();
        //println!("Found low points: {:?}", low_points);
        let mut basins: Vec<Vec<(i64, i64)>> = vec![];

        for (x, y) in low_points.iter() {
            let mut basin: HashSet<(i64, i64)> = HashSet::new();
            basin.insert((*x, *y));
            //println!("Looking for basin for ({},{})", x, y);
            self.find_basin(&mut basin, *x, *y);

            if basin.iter().count() > 0 {
                basins.push(basin.into_iter().collect::<Vec<_>>());
            }
        } //End low point loop
        basins
    }

    fn is_low_point(&self, x: i64, y: i64) -> bool {
        let val = self.get(x, y).unwrap();
        for cx in x - 1..x + 2 {
            for cy in y - 1..y + 2 {
                if cx == x && cy == y {
                    continue;
                }
                if let Some(c) = self.get(cx, cy) {
                    if c <= val {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn risk_level(&self, x: i64, y: i64) -> i64 {
        assert!(self.is_low_point(x, y));
        self.get(x, y).unwrap() as i64 + 1
    }

    fn find_low_points(&self) -> Vec<(i64, i64)> {
        let mut out_vec = vec![];
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                if self.is_low_point(x, y) {
                    out_vec.push((x, y));
                }
            }
        }
        out_vec
    }

    fn get(&self, x: i64, y: i64) -> Option<i64> {
        //NOTE: This is not stored how you think it is, so you grab it inverted
        match self.map.get(y as usize) {
            Some(row) => match row.get(x as usize) {
                Some(item) => Some(*item),
                None => None,
            },
            None => None,
        }
    }
}

impl fmt::Display for Map {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let val = match self.get(x, y) {
                    Some(x) => x.to_string(),
                    None => "X".to_string(),
                };
                write!(f, "({},{},{:?})", x, y, val)?;

                //write!(f, "{}", val);
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[aoc_generator(day9)]
fn day9_parse(input: &str) -> Map {
    Map::new(input)
}

#[aoc(day9, part1)]
pub fn day9_part1(input: &Map) -> u128 {
    //print!("{}", input);
    let low_points = input.find_low_points();
    //println!("Low points: {:?}", low_points);
    low_points
        .iter()
        .map(|(x, y)| input.risk_level(*x, *y))
        .sum::<i64>() as u128
}

#[aoc(day9, part2)]
pub fn day9_part2(input: &Map) -> u128 {
    let basins = input.find_basins();
    //println!("{:?}", basins);
    let mut basin_sizes = basins
        .iter()
        .map(|basin| basin.iter().count())
        .collect::<Vec<_>>();
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).fold(1, |a, x| a * x) as u128
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "2199943210
            3987894921
9856789892
8767896789
9899965678"
    }

    #[test]
    fn day9_part1_test() {
        assert_eq!(day9_part1(&day9_parse(get_test_input())), 15);
    }
    #[test]
    fn day9_part2_test() {
        assert_eq!(day9_part2(&day9_parse(get_test_input())), 1134);
    }
}
