use crate::Day;

pub struct Day2 {}

impl Day<usize> for Day2 {
    fn number() -> usize {
        2
    }

    fn run_star1(file: String) -> usize {
        parse_line(&file).filter(|line| is_valid_line(line)).count()
    }

    fn run_star2(file: String) -> usize {
        parse_line(&file)
            .filter(|line| is_valid_line_with_tolerance(line))
            .count()
    }
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
    use crate::day2::Day2;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day2::run_star1(Day2::get_example());
        assert_eq!(result, 2);
    }

    #[test]
    fn example_star2() {
        let result = Day2::run_star2(Day2::get_example());
        assert_eq!(result, 4);
    }
}
