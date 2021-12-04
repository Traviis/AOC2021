#[derive(Debug)]
enum CellState {
    Marked(u64),
    Unmarked(u64),
}

//Don't have to, but let's do this OO
#[derive(Debug)]
struct Board {
    data: Vec<Vec<CellState>>,
    has_won: bool,
}

impl Board {
    fn new(input: Vec<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let vec_lines = input
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(str::trim)
                    .map(str::parse::<u64>)
                    .map(|num| CellState::Unmarked(num.unwrap()))
                    .collect::<Vec<CellState>>()
            })
            .collect::<Vec<_>>();

        Ok(Board {
            data: vec_lines,
            has_won: false,
        })
    }

    fn try_mark(&mut self, mark: u64) -> Option<(u64, u64)> {
        for (row_idx, row) in self.data.iter_mut().enumerate() {
            for (col_idx, item) in row.iter_mut().enumerate() {
                if let CellState::Unmarked(n) = item {
                    if *n == mark {
                        *item = CellState::Marked(*n);
                        return Some((row_idx as u64, col_idx as u64));
                    }
                }
            }
        }

        None
    }

    fn unmarked_sum(&self) -> u64 {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| match val {
                        CellState::Unmarked(n) => *n,
                        _ => 0,
                    })
                    .sum::<u64>()
            })
            .sum::<u64>()
    }

    fn check_win(&self, x: u64, y: u64) -> Option<bool> {
        //Check Horizontal wins
        let mut col_unmarked = false;
        for col in 0..5 {
            if let CellState::Unmarked(_) = self.data[x as usize][col as usize] {
                col_unmarked = true;
                break;
            }
        }
        //TODO: Make this cleaner
        if !col_unmarked {
            return Some(true); //Win
        }
        for row in 0..5 {
            if let CellState::Unmarked(_) = self.data[row as usize][y as usize] {
                return None;
            }
        }
        return Some(true);
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Board>) {
    let mut in_iter = input.split("\n\n");

    let calls = in_iter
        .next()
        .unwrap()
        .split(",")
        .map(str::trim)
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();

    let boards = in_iter
        .map(|board_raw| Board::new(board_raw.split("\n").collect::<Vec<_>>()).unwrap())
        .collect::<Vec<_>>();

    (calls, boards)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let (calls, mut boards) = parse_input(input);

    let mut winning_boards = 0;

    let total_boards = boards.iter().count();

    for &call in calls.iter() {
        for board in boards.iter_mut() {
            if board.has_won {
                continue;
            }
            if let Some((x, y)) = board.try_mark(call) {
                if let Some(_) = board.check_win(x, y) {
                    board.has_won = true;
                    winning_boards += 1;
                    if winning_boards == total_boards {
                        return board.unmarked_sum() * call;
                    }
                }
            }
        }
    }

    0
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    let (calls, mut boards) = parse_input(input);

    for &call in calls.iter() {
        for board in boards.iter_mut() {
            if let Some((x, y)) = board.try_mark(call) {
                if let Some(_) = board.check_win(x, y) {
                    return board.unmarked_sum() * call;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(get_test_input()), 4512);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(get_test_input()), 1924);
    }
}
