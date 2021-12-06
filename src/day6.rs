use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|number| number.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(initial_population: &[usize]) -> usize {
    reproduction(initial_population, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(initial_population: &[usize]) -> usize {
    reproduction(initial_population, 256)
}

fn reproduction(initial_population: &[usize], generations: usize) -> usize {
    let mut groups = [0; 9];
    for individual in initial_population {
        groups[*individual] += 1;
    }

    for reproducing in (0..=6).cycle().take(generations) {
        let newborns = groups[reproducing];
        groups[reproducing] += groups[7];
        groups[7] = groups[8];
        groups[8] = newborns;
    }

    groups.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator};

    static SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn solve_sample_1() {
        let result = solve_part1(&input_generator(SAMPLE));
        assert_eq!(5934, result);
    }

    #[test]
    fn solve_sample_2() {
        let result = solve_part2(&input_generator(SAMPLE));
        assert_eq!(26984457539, result);
    }
}
