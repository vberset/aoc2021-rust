use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.bytes().map(|bit| (bit - b'0') as usize).collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    let bits_count = input[0].len();
    let threshold = input.len() / 2;
    let gamma = input
        .iter()
        .fold(vec![0; bits_count], |counters, line| counters.iter().zip(line).map(|(a, b)| a + b).collect())
        .iter()
        .fold(0, |gamma, counter| gamma * 2 + ((*counter > threshold) as usize));
    let mask = (1 << bits_count) - 1;
    let epsilon = mask ^ gamma;
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {
    let oxygen = search(input, true);
    let co2 = search(input, false);
    oxygen * co2
}

fn search(lines: &[Vec<usize>], majority: bool) -> usize {
    let mut lines = lines.iter().collect::<Vec<_>>();

    for i in 0.. {
        let ones = lines.iter().fold(0, |count, line| count + line[i]);
        let criteria = ((ones >= lines.len() - ones) ^ !majority) as usize;
        lines = lines.into_iter().filter(|line| line[i] == criteria).collect();
        if lines.len() == 1 {
            break;
        }
    }

    lines[0].iter().fold(0, |number, bit| number * 2 + bit)
}

#[cfg(test)]
mod tests {
    use super::solve_part1;
    use super::solve_part2;
    use super::input_generator;

    static SAMPLE: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn solve_sample_1() {
        let result = solve_part1(&input_generator(SAMPLE));
        assert_eq!(198, result);
    }

    #[test]
    fn solve_sample_2() {
        let result = solve_part2(&input_generator(SAMPLE));
        assert_eq!(230, result);
    }
}
