use aoc_runner_derive::{aoc, aoc_generator};

pub enum Move {
    Horizontal(i32),
    Vertical(i32),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let direction = splits.next().unwrap();
            let distance = splits.next().unwrap().parse().unwrap();
            match direction {
                "forward" => Move::Horizontal(distance),
                "up" => Move::Vertical(-distance),
                "down" => Move::Vertical(distance),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Move]) -> i32 {
    let (h, v) = input
        .iter()
        .fold((0, 0), |(h, v), mve| match mve {
            Move::Horizontal(dist) => (h + dist, v),
            Move::Vertical(dist) => (h, v + dist),
        });
    h * v
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Move]) -> i32 {
    let (h, v, _) = input
        .iter()
        .fold((0, 0, 0), |(h, v, aim), mve| match mve {
            Move::Horizontal(dist) => (h + dist, v + dist * aim, aim),
            Move::Vertical(dist) => (h, v, aim + dist),
        });
    h * v
}
