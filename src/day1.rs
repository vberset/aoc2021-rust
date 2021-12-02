use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    input
        .windows(2)
        .filter(|ab| ab[1] > ab[0])
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> usize {
    input
        .windows(4)
        .filter(|window| window[3] > window[0])
        .count()
}
