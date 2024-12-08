use crate::Day;

pub struct Day7 {}
impl Day<usize> for Day7 {
    fn number() -> usize {
        7
    }

    fn run_star1(file: String) -> usize {
        let equations = parse_input(file);

        equations
            .iter()
            .filter(|equation| equation.is_valid(false))
            .map(|equation| equation.result)
            .sum()
    }

    fn run_star2(file: String) -> usize {
        let equations = parse_input(file);

        equations
            .iter()
            .filter(|equation| equation.is_valid(true))
            .map(|equation| equation.result)
            .sum()
    }
}

pub struct Equation {
    result: usize,
    values: Vec<usize>,
}

impl Equation {
    fn is_valid(&self, allow_concatenation: bool) -> bool {
        self.check_valid(self.values[0], 1, allow_concatenation)
    }

    fn check_valid(&self, temp_result: usize, index: usize, allow_concatenation: bool) -> bool {
        if index >= self.values.len() {
            return temp_result == self.result;
        }

        if self.check_valid(
            temp_result * self.values[index],
            index + 1,
            allow_concatenation,
        ) {
            return true;
        }

        if self.check_valid(
            temp_result + self.values[index],
            index + 1,
            allow_concatenation,
        ) {
            return true;
        }

        if allow_concatenation
            && self.check_valid(
                (temp_result.to_string() + &self.values[index].to_string())
                    .parse()
                    .unwrap(),
                index + 1,
                allow_concatenation,
            )
        {
            return true;
        }

        false
    }
}
fn parse_input(file: String) -> Vec<Equation> {
    file.lines()
        .map(|line| {
            let mut split = line.split(':');

            let result = split.next().unwrap().parse().unwrap();

            let values = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            Equation { result, values }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day7::Day7;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day7::run_star1(Day7::get_example());
        assert_eq!(result, 3749);
    }

    #[test]
    fn example_star2() {
        let result = Day7::run_star2(Day7::get_example());
        assert_eq!(result, 11387);
    }
}
