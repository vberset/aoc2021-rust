use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

type Board = Vec<Vec<usize>>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut lines_iterator = input.split("\n\n");
    let draws = lines_iterator.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    let boards = lines_iterator.map(|board| board.lines().map(|line| line.split_whitespace().map(|case| case.parse().unwrap()).collect()).collect()).collect();
    (draws, boards)
}

#[aoc(day4, part1)]
pub fn solve_part1((draws, boards): &(Vec<usize>, Vec<Board>)) -> usize {
    let mut states = boards.iter().map(|board| BoardState::new(board)).collect::<Vec<_>>();
    let reverse_index = build_inverted_index(&boards);

    for draw in draws {
        for (iboard, irow, icol) in reverse_index.get(draw).unwrap() {
            if let Some(score) = states[*iboard].marks(*draw, *irow, *icol) {
                return score;
            }
        }
    }
    unreachable!();
}

#[aoc(day4, part2)]
pub fn solve_part2((draws, boards): &(Vec<usize>, Vec<Board>)) -> usize {
    let mut states = boards.iter().map(|board| BoardState::new(board)).collect::<Vec<_>>();
    let inverted_index = build_inverted_index(&boards);
    let mut remaining_boards = (0..boards.len()).collect::<HashSet<usize>>();

    for draw in draws {
        for (iboard, irow, icol) in inverted_index.get(draw).unwrap() {
            if let Some(score) = states[*iboard].marks(*draw, *irow, *icol) {
                remaining_boards.remove(iboard);
                if remaining_boards.is_empty() {
                    return score;
                }
            }
        }
    }
    unreachable!();
}

fn build_inverted_index(boards: &[Board]) -> HashMap<usize, Vec<(usize, usize, usize)>> {
    let mut inverted_index = HashMap::new();
    for (iboard, board) in boards.iter().enumerate() {
        for (irow, row) in board.iter().enumerate() {
            for (icol, value) in row.iter().enumerate() {
                inverted_index.entry(*value).or_insert_with(Vec::new).push((iboard, irow, icol));
            }
        }
    }
    inverted_index
}

struct BoardState {
    remaining: usize,
    rows: Vec<usize>,
    columns: Vec<usize>,
}

impl BoardState {
    fn new(board: &Board) -> Self {
        let height = board.len();
        let width = board[0].len();
        Self {
            remaining: board.iter().flatten().sum(),
            rows: vec![width; height],
            columns: vec![height; width],
        }
    }

    fn marks(&mut self, value: usize, row: usize, column: usize) -> Option<usize> {
        self.remaining -= value;
        self.rows[row] -= 1;
        self.columns[column] -= 1;
        if self.rows[row] == 0 || self.columns[column] == 0 {
            Some(value * self.remaining)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_part1;
    use super::solve_part2;
    use super::input_generator;

    static SAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7"#;

    #[test]
    fn solve_sample_1() {
        let result = solve_part1(&input_generator(SAMPLE));
        assert_eq!(4512, result);
    }

    #[test]
    fn solve_sample_2() {
        let result = solve_part2(&input_generator(SAMPLE));
        assert_eq!(1924, result);
    }
}
