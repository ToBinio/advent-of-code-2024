use crate::Day;
use std::collections::{HashMap, HashSet};
use std::ops::Not;

pub struct Day10 {}
impl Day<usize> for Day10 {
    fn number() -> usize {
        10
    }

    fn run_star1(file: String) -> usize {
        let map = parse_input(&file);

        let width = map[0].len();
        let height = map.len();

        let mut count = 0;

        for y in 0..height {
            for x in 0..width {
                if map[y][x] != 0 {
                    continue;
                }

                let vec = get_peak_count(&map, x, y);
                let mut set = HashSet::new();
                set.extend(vec);
                count += set.len();
            }
        }

        count
    }

    fn run_star2(file: String) -> usize {
        let map = parse_input(&file);

        let width = map[0].len();
        let height = map.len();

        let mut count = 0;

        for y in 0..height {
            for x in 0..width {
                if map[y][x] != 0 {
                    continue;
                }

                let vec = get_peak_count(&map, x, y);
                count += vec.len();
            }
        }

        count
    }
}

fn get_peak_count(map: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let width = map[0].len();
    let height = map.len();

    let mut peaks = Vec::new();

    if (map[y][x] == 9) {
        peaks.push((x, y));
        return peaks;
    }

    if x > 0 && is_valid_slope(map, (x, y), (x - 1, y)) {
        let new_peaks = get_peak_count(&map, x - 1, y);
        peaks.extend(new_peaks);
    }

    if y > 0 && is_valid_slope(map, (x, y), (x, y - 1)) {
        let new_peaks = get_peak_count(&map, x, y - 1);
        peaks.extend(new_peaks);
    }

    if x < (width - 1) && is_valid_slope(map, (x, y), (x + 1, y)) {
        let new_peaks = get_peak_count(&map, x + 1, y);
        peaks.extend(new_peaks);
    }

    if y < (height - 1) && is_valid_slope(map, (x, y), (x, y + 1)) {
        let new_peaks = get_peak_count(&map, x, y + 1);
        peaks.extend(new_peaks);
    }

    peaks
}

fn is_valid_slope(map: &Vec<Vec<usize>>, from: (usize, usize), to: (usize, usize)) -> bool {
    map[to.1][to.0] > map[from.1][from.0] && (map[to.1][to.0] - map[from.1][from.0]) == 1
}

fn parse_input(file: &str) -> Vec<Vec<usize>> {
    file.lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day10::Day10;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day10::run_star1(Day10::get_example());
        assert_eq!(result, 36);
    }

    #[test]
    fn example_star2() {
        let result = Day10::run_star2(Day10::get_example());
        assert_eq!(result, 81);
    }
}
