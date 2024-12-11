use crate::Day;
use std::collections::{HashMap, HashSet};
use std::ops::Not;

pub struct Day11 {}
impl Day<usize> for Day11 {
    fn number() -> usize {
        11
    }

    fn run_star1(file: String) -> usize {
        let mut stones = parse_input(&file);

        for _ in 0..25 {
            stones = step(&mut stones);
        }

        stones.iter().map(|(_, value)| value).sum()
    }

    fn run_star2(file: String) -> usize {
        let mut stones = parse_input(&file);

        for _ in 0..75 {
            stones = step(&mut stones);
        }

        stones.iter().map(|(_, value)| value).sum()
    }
}

fn parse_input(file: &str) -> HashMap<usize, usize> {
    file.split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_default() += 1;
            acc
        })
}

fn step(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();

    for (value, count) in stones {
        if *value == 0 {
            *new_stones.entry(1).or_default() += *count;
            continue;
        }

        let string_rep = value.to_string();
        if string_rep.len() % 2 == 0 {
            let first_part = string_rep[0..string_rep.len() / 2]
                .parse::<usize>()
                .unwrap();
            let second_part = string_rep[(string_rep.len() / 2)..string_rep.len()]
                .parse::<usize>()
                .unwrap();

            *new_stones.entry(first_part).or_default() += count;
            *new_stones.entry(second_part).or_default() += count;

            continue;
        }

        *new_stones.entry(value * 2024).or_default() += *count;
    }

    new_stones
}

#[cfg(test)]
mod tests {
    use crate::day11::Day11;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day11::run_star1(Day11::get_example());
        assert_eq!(result, 55312);
    }
}
