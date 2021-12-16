use std::collections::{HashMap, HashSet};

type Tile = Vec<Vec<i64>>;

#[aoc_generator(day15)]
fn day15_parse(input: &str) -> Tile {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string())
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .collect::<Tile>()
}

//Lame to input max_x and max_y here, but don't want to recalc every time
fn get_adjacent_nodes(vec_map: &Tile, x: i64, y: i64, max_x: i64, max_y: i64) -> Vec<(i64, i64)> {
    let transform_map = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    transform_map
        .iter()
        .map(|(dx, dy)| {
            let tx = dx + x;
            let ty = dy + y;
            if tx < 0 || tx >= max_x || ty < 0 || ty >= max_y {
                return None;
            }
            return Some((tx, ty));
        })
        .filter(|x| x.is_some())
        .flatten()
        .collect::<Vec<(i64, i64)>>()
}

#[aoc(day15, part1)]
pub fn day15_part1(vec_map: &Tile) -> u128 {
    dij(false, vec_map)
}

#[aoc(day15, part2)]
pub fn day15_part2(vec_map: &Tile) -> u128 {
    dij(true, vec_map)
}

fn mutate_tile(vec_map: &Tile) -> Tile {
    // +1, wrap 10s to 1
    let mut mut_map = vec_map.clone();
    for y in 0..vec_map.iter().count() {
        for x in 0..vec_map[y].iter().count() {
            mut_map[y][x] = match vec_map[y][x] {
                n if n < 9 => n + 1,
                9 => 1,
                _ => panic!(),
            }
        }
    }

    mut_map
}

fn expand_map(vec_map: &Tile, max_x: i64, max_y: i64) -> Tile {
    let mut meta_map: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut meta_tile = vec_map.clone();

    meta_map.insert((0,0), vec_map.clone());

    //Each original vec_map is the (0,0) tile in a 5 by 5 grid
    for meta_y in 0..5 {
        for meta_x in 0..5 {
            if meta_x == 0 && meta_y == 0 {
                continue; //starting case
            }
            //Find the source tile, prefer left, but if it's unavailable, use top
            let (source_x, source_y) = if meta_x == 0 {
                (meta_x, meta_y - 1)
            } else {
                (meta_x - 1, meta_y)
            };

            //println!("Source to mutate: ({},{})", source_x,source_y);
            //println!("Meta Map keys: {:?}", meta_map.keys());
            let new_tile = mutate_tile(&meta_map.get(&(source_x, source_y)).unwrap());
            for t_y in 0..max_y {
                for t_x in 0..max_x {
                    //Don't care about x, since we are pushing
                    let cur_y = t_y as usize + (meta_y * max_y as usize);
                    meta_tile[cur_y].push(new_tile[t_y as usize][t_x as usize]);
                }
            }

            meta_map.insert((meta_x, meta_y), new_tile);
        }

//TODO
        if meta_y != 4 { //Off by one error? Never heard of her 
            for _ in 0..max_y {
                meta_tile.push(Vec::new()); //expand down
            }
        }
    }

    meta_tile
}

pub fn dij(part_2: bool, vec_map: &Tile) -> u128 {
    //This is slightly modified djikstras. Each node is the (x,y) coordinate, however, the edge
    //value is actually the value of the node (and all edges coming into that node have that same
    //value
    //
    let mut vec_map = vec_map.clone();

    let mut max_x = vec_map[0].iter().count() as i64;
    let mut max_y = vec_map.iter().count() as i64;

    if part_2 {
        vec_map = expand_map(&vec_map, max_x, max_y);
        max_x = vec_map[0].iter().count() as i64;
        max_y = vec_map.iter().count() as i64;
    }

/*
    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", vec_map[y as usize][x as usize]);
        }
        print!("\n");
    }
    */

    //Non-existence implies infinite
    //It also implies visitation if it's in there
    let mut dist_map: HashMap<(i64, i64), i64> = HashMap::new();
    //let mut bin_heap = BinaryHeap::new();

    //https://www.programiz.com/dsa/dijkstra-algorithm
    //
    //Using MAX - 50 because we can overflow in some cases, so let's just cheat a bit

    for y in 0..max_y {
        for x in 0..max_x {
            //TODO: Can I perhaps skip this, and just imply it's infinite if it's not in this map?
            dist_map.entry((x, y)).or_insert(i64::MAX - 50);
            let v = vec_map[y as usize][x as usize];
        }
    }

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    //Set the initial to 0
    *dist_map.entry((0, 0)).or_insert(0) = 0;

    loop {
        //https://www.linkedin.com/pulse/practical-dijkstras-algorithm-farruh-habibullaev
        // the u in question

        //TODO: Optimize this, for every single iteration here, I'm filtering on all of the
        //dist_map. That's bad. If I remove the infinite insertion, and instead rely on the
        //non-existence equaling infinite. Then dist_map will be orders of magnitude smaller, even
        //if I end up iterating over it. Really, this would be best as a min-heap.
        //
        //TODO: 2, perhaps if I convert dist_map to a min-heap, then I can immediately pop those
        //nodes any time (custom structure that impl Ord); The annoying thing with that is that I
        //look for neighbor values later. Perhaps keep a map AND a min-heap?
        let (&(x, y), &v) = dist_map
            .iter()
            .filter(|((x, y), _)| !visited.contains(&(*x, *y)))
            .min_by_key(|((x, y), v)| *v)
            .unwrap()
            .clone();

        visited.insert((x, y));

        //println!("Looking at Node: ({},{})", x, y);
        //Get only unvisited nodes
        let neighbors = get_adjacent_nodes(&vec_map, x, y, max_x, max_y)
            .into_iter()
            //Unvisited
            .filter(|(x, y)| *dist_map.get_key_value(&(*x, *y)).unwrap().1 == i64::MAX - 50)
            .collect::<Vec<(i64, i64)>>();
        //println!("Neighbors: {:?}", neighbors);

        for (nx, ny) in neighbors.iter() {
            //println!("Checking ({},{}) => {}", nx, ny, vec_map[*ny as usize][*nx as usize]);
            let t_dist = v + vec_map[*ny as usize][*nx as usize];

            if t_dist < *dist_map.get(&(*nx, *ny)).unwrap() {
                *dist_map.entry((*nx, *ny)).or_insert(-1) = t_dist;
            }
        }

        if visited.iter().count() == (max_x * max_y) as usize {
            break;
        }
    }

    /*
    for ((x, y), v) in dist_map.iter() {
        println!("({},{}) -> {}", x, y, v);
    }
    */
    *dist_map.get(&(max_x - 1, max_y - 1)).unwrap() as u128
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581"
    }

    #[test]
    fn day15_part1() {
        assert_eq!(super::day15_part1(&day15_parse(get_test_input())), 40);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(super::day15_part2(&day15_parse(get_test_input())), 315);
    }
}
