use crate::Day;
use regex::Regex;

pub struct Day4 {}

const HORIZONTAL_OFFSET: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const VERTICAL_OFFSET: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];

const DIAGONAL_RIGHT_OFFSET: [(usize, usize); 4] = [(0, 0), (1, 1), (2, 2), (3, 3)];

const DIAGONAL_LEFT_OFFSET: [(usize, usize); 4] = [(0, 3), (1, 2), (2, 1), (3, 0)];

const SMALL_DIAGONAL_LEFT_OFFSET: [(usize, usize); 4] = [(0, 2), (1, 1), (2, 0), (0, 0)];

impl Day<usize> for Day4 {
    fn number() -> usize {
        4
    }

    fn run_star1(file: String) -> usize {
        let data = parse_input(file);

        let mut count = 0;

        for y in 0..data.len() {
            for x in 0..data[0].len() {
                if x < data[0].len() - 3 {
                    if checkText("XMAS", &data, (x, y), HORIZONTAL_OFFSET, false) {
                        count += 1;
                    };

                    if checkText("XMAS", &data, (x, y), HORIZONTAL_OFFSET, true) {
                        count += 1;
                    };
                }

                if y < data.len() - 3 {
                    if checkText("XMAS", &data, (x, y), VERTICAL_OFFSET, false) {
                        count += 1;
                    };

                    if checkText("XMAS", &data, (x, y), VERTICAL_OFFSET, true) {
                        count += 1;
                    };
                }

                if y < data.len() - 3 && x < data[0].len() - 3 {
                    if checkText("XMAS", &data, (x, y), DIAGONAL_RIGHT_OFFSET, false) {
                        count += 1;
                    };

                    if checkText("XMAS", &data, (x, y), DIAGONAL_RIGHT_OFFSET, true) {
                        count += 1;
                    };

                    if checkText("XMAS", &data, (x, y), DIAGONAL_LEFT_OFFSET, false) {
                        count += 1;
                    };

                    if checkText("XMAS", &data, (x, y), DIAGONAL_LEFT_OFFSET, true) {
                        count += 1;
                    };
                }
            }
        }

        count
    }

    fn run_star2(file: String) -> usize {
        let data = parse_input(file);

        let mut count = 0;

        for y in 0..(data.len() - 2) {
            for x in 0..(data[0].len() - 2) {
                let has_right_diagonal =
                    checkText("MAS", &data, (x, y), DIAGONAL_RIGHT_OFFSET, false)
                        || checkText("MAS", &data, (x, y), DIAGONAL_RIGHT_OFFSET, true);

                let has_left_diagonal =
                    checkText("MAS", &data, (x, y), SMALL_DIAGONAL_LEFT_OFFSET, false)
                        || checkText("MAS", &data, (x, y), SMALL_DIAGONAL_LEFT_OFFSET, true);

                if has_right_diagonal && has_left_diagonal {
                    count += 1;
                }
            }
        }

        count
    }
}

fn parse_input(file: String) -> Vec<Vec<char>> {
    file.lines().map(|line| line.chars().collect()).collect()
}

fn checkText(
    string: &str,
    data: &Vec<Vec<char>>,
    pos: (usize, usize),
    offset: [(usize, usize); 4],
    inverse: bool,
) -> bool {
    for (index, char) in string.chars().enumerate() {
        let index = if inverse {
            string.len() - index - 1
        } else {
            index
        };

        if data[pos.1 + offset[index].1][pos.0 + offset[index].0] != char {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::day4::Day4;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day4::run_star1(Day4::get_example());
        assert_eq!(result, 18);
    }

    #[test]
    fn example_star2() {
        let result = Day4::run_star2(Day4::get_example());
        assert_eq!(result, 9);
    }
}
