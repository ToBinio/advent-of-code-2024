use std::iter::Map;
use std::str::Lines;

pub fn run_star1(file: String) -> usize {
    parse_line(&file).filter(|line| is_valid_line(line)).count()
}

pub fn run_star2(file: String) -> usize {
    parse_line(&file)
        .filter(|line| is_valid_line_with_tolerance(line))
        .count()
}

fn parse_line(file: &String) -> impl Iterator<Item = Vec<usize>> + '_ {
    file.lines().map(|line| {
        line.split(" ")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    })
}

fn is_valid_line(line: &Vec<usize>) -> bool {
    for i in 1..line.len() {
        let diff = line[i - 1].abs_diff(line[i]);

        if diff == 0 || diff > 3 {
            return false;
        }
    }

    increasing(line) || decreasing(line)
}

fn is_valid_line_with_tolerance(line: &Vec<usize>) -> bool {
    if is_valid_line(line) {
        return true;
    }

    for i in 0..line.len() {
        let mut new_line = line.clone();
        new_line.remove(i);

        if is_valid_line(&new_line) {
            return true;
        }
    }

    false
}

fn increasing(line: &Vec<usize>) -> bool {
    for i in 1..line.len() {
        if line[i] <= line[i - 1] {
            return false;
        }
    }

    true
}

fn decreasing(line: &Vec<usize>) -> bool {
    for i in 1..line.len() {
        if line[i] >= line[i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::day2::{run_star1, run_star2};
    use crate::read_file;

    #[test]
    fn example_star1() {
        let result = run_star1(read_file(2, true));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_star2() {
        let result = run_star2(read_file(2, true));
        assert_eq!(result, 4);
    }
}
