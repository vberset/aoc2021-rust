use std::collections::HashSet;
use std::cmp::Ordering::*;
use std::iter::repeat;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input.lines().map(|line| {
        let mut parts = line.split("->")
            .map(|couple| couple.split(",").map(|number| number.trim().parse().unwrap()))
            .flatten();
        ((parts.next().unwrap(), parts.next().unwrap()), (parts.next().unwrap(), parts.next().unwrap()))
    }).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(lines: &[((usize, usize), (usize, usize))]) -> usize {
    let mut points = HashSet::new();
    let mut overlaps = HashSet::new();
    for ((a_x, a_y), (b_x, b_y)) in lines.iter().cloned() {
        if a_x == b_x || a_y == b_y {
            for point in range(a_x, b_x).zip(range(a_y, b_y)) {
                if !points.insert(point) {
                    overlaps.insert(point);
                }
            }
        }
    }
    overlaps.len()
}

#[aoc(day5, part2)]
pub fn solve_part2(lines: &[((usize, usize), (usize, usize))]) -> usize {
    let mut points = HashSet::new();
    let mut overlaps = HashSet::new();
    for ((a_x, a_y), (b_x, b_y)) in lines.iter().cloned() {
        for point in range(a_x, b_x).zip(range(a_y, b_y)) {
            if !points.insert(point) {
                overlaps.insert(point);
            }
        }
    }
    overlaps.len()
}

fn range(a: usize, b: usize) -> Box<dyn Iterator<Item=usize>> {
    match a.cmp(&b) {
        Equal => Box::new(repeat(a)),
        Less => Box::new(a..=b),
        Greater => Box::new((b..=a).rev()),
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator};

    static SAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn solve_sample_1() {
        let result = solve_part1(&input_generator(SAMPLE));
        assert_eq!(5, result);
    }

    #[test]
    fn solve_sample_2() {
        let result = solve_part2(&input_generator(SAMPLE));
        assert_eq!(12, result);
    }
}


