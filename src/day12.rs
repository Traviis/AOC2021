use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Cave {
    Small(String),
    Big(String),
    Start,
    End,
}

fn parse_segment(seg: &str) -> Cave {
    match seg {
        "start" => Cave::Start,
        "end" => Cave::End,
        o => {
            if o.chars().all(|c| c.is_uppercase()) {
                Cave::Big(o.into())
            } else {
                Cave::Small(o.into())
            }
        }
    }
}

#[aoc_generator(day12)]
fn day12_parse(input: &str) -> HashMap<Cave, Vec<Cave>> {
    let mut segments: HashMap<Cave, Vec<Cave>> = HashMap::new();
    for line in input.lines().map(str::trim) {
        let mut parts = line.split("-");
        let beg = parse_segment(parts.next().unwrap());
        let end = parse_segment(parts.next().unwrap());

        if end != Cave::Start && beg != Cave::End {
            let ent = segments.entry(beg.clone()).or_insert(vec![]);
            ent.push(end.clone());
        }
        //Backlinks!
        if end != Cave::End && beg != Cave::Start {
            let ent2 = segments.entry(end).or_insert(vec![]);
            ent2.push(beg);
        }
    }

    segments
}

/// For every call of bfs, take the current_position, and return the set of posibile next
/// locations, for small caves, check if you have already hit it.
fn bfs(
    p2_variant: bool,
    already_visited: &Vec<Cave>,
    segments: &HashMap<Cave, Vec<Cave>>,
) -> Vec<Vec<Cave>> {
    let mut all_paths: Vec<Vec<Cave>> = vec![];
    let pos = already_visited.last().unwrap();

    //println!("p2: {:?} Current Path {:?}", p2_variant, already_visited);
    let possible_caves = segments.get(&pos);

    if let None = possible_caves {
        //println!("Bailing, no possibilities");
        return all_paths;
    }

    //Can pre-filter here to not even bother checking next_cave for small caves we have already
    //visited
    let filtered_caves = match p2_variant {
        false => possible_caves
            .unwrap()
            .iter()
            .filter(|nc| {
                !already_visited
                    .iter()
                    .filter(|f| if let Cave::Small(_) = f { true } else { false })
                    .any(|c| *c == **nc)
            })
            .map(|x| x.clone())
            .collect::<Vec<Cave>>(),
        true => possible_caves.unwrap().clone(), //TODO: refactor
    };

    //println!("Filtered {:?}", filtered_caves);

    for next_cave in filtered_caves.iter() {
        match next_cave {
            Cave::Start => {
                panic!("Shouldn't happen");
            }
            Cave::End => {
                let mut al_vis = already_visited.clone();
                al_vis.push(Cave::End);
                all_paths.push(al_vis);
                //println!("");
                continue;
            }
            Cave::Big(_) | Cave::Small(_) => {
                if p2_variant {
                    //Check if the next node is either the only repeated one (great) or if it's not
                    //in the list at all
                    if let Cave::Small(_) = next_cave {
                        let mut small_freq = HashMap::new();
                        already_visited
                            .iter()
                            .filter(|f| if let Cave::Small(_) = f { true } else { false })
                            .for_each(|av| {
                                *small_freq.entry(av).or_insert(0) += 1;
                            });
                        //You can repeat if no other small cave has repeated already
                        let a_cave_can_repeat = !small_freq.iter().any(|(_, &v)| v == 2);
                        let this_node_exists = already_visited.iter().any(|f| *f == *next_cave);
                        //You can move forward if you don't exist,
                        // or if you do exist, a small cave can repeat (no other small cave is
                        // repeated),
                        // if another node already exists, and you already exist, you can't continue
                        if !this_node_exists || (a_cave_can_repeat && this_node_exists) {
                            //println!("Can repeat {:?}", next_cave);
                        } else {
                            continue;
                        }
                    }
                }

                let cav = next_cave.clone();
                let mut al_vis = already_visited.clone();
                al_vis.push(cav.clone()); //Really, you only need to track small caves

                let mut path_seg = bfs(p2_variant, &al_vis, &segments);
                //           println!("Next Valid Segments: ");

                for possibility in path_seg.iter_mut() {
                    //                println!("{:?}", possibility);
                    if possibility.iter().count() == 0 {
                        continue;
                    }
                    all_paths.push(possibility.clone());
                }
            }
        };
    }
    all_paths
}

#[aoc(day12, part1)]
pub fn day12_part1(map: &HashMap<Cave, Vec<Cave>>) -> u128 {
    path_finder(false, &map)
}
#[aoc(day12, part2)]
pub fn day12_part2(map: &HashMap<Cave, Vec<Cave>>) -> u128 {
    path_finder(true, &map)
}
pub fn path_finder(part2_variant: bool, map: &HashMap<Cave, Vec<Cave>>) -> u128 {
    //println!("{:?}", map);
    //for (k, v) in map.iter() {
    //    println!("{:?} => {:?}", k, v);
    //}
    let paths = bfs(part2_variant, &vec![Cave::Start], &map);
    //for path in paths.iter() {
    //    println!("Path {:?}", path);
    //}
    paths.iter().count() as u128
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end"
    }

    #[test]
    fn day12_part1() {
        assert_eq!(super::day12_part1(&day12_parse(get_test_input())), 10);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(super::day12_part2(&day12_parse(get_test_input())), 36);
    }
}
