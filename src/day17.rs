use log::debug;

#[aoc_generator(day17)]
fn day17_parse(input: &str) -> (i64, i64, i64, i64) {
    let mut v = input.split(" ");
    let xeq = v.nth(2).unwrap();
    // Eh, screw it.
    // No checking errors here, we unwrap like men
    let min_x = xeq
        .split("..")
        .next()
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    // Olympics for most obscure parsing!
    let max_x = xeq
        .split("..")
        .nth(1)
        .unwrap()
        .split(",")
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let yeq = input.split(" ").nth(3).unwrap();
    let y_vals = yeq.split("=").skip(1).collect::<String>();

    let min_y = y_vals.split("..").nth(0).unwrap().parse().unwrap();
    let max_y = y_vals.split("..").nth(1).unwrap().parse().unwrap();

    //I'm just dicking around at this point
    (min_x, max_x, min_y, max_y)
}

#[aoc(day17, part1)]
pub fn day17_part1((min_x, max_x, min_y, max_y): &(i64, i64, i64, i64)) -> i64 {
    debug!(
        "min_x {}, max_x {} , min_y {}, max_y {}",
        min_x, max_x, min_y, max_y
    );
    (-min_y - 1) * (-min_y / 2)
}

#[aoc(day17, part2)]
pub fn day17_part2((min_x, max_x, min_y, max_y): &(i64, i64, i64, i64)) -> u128 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_log::test;

    #[test]
    fn day17_part1() {
        let inp = "target area: x=20..30, y=-10..-5";
        assert_eq!(super::day17_part1(&day17_parse(inp)),45);
    }
}
