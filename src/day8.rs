use crate::Day;
use std::collections::{HashMap, HashSet};

pub struct Day8 {}
impl Day<usize> for Day8 {
    fn number() -> usize {
        8
    }

    fn run_star1(file: String) -> usize {
        let frequencies = parse_input(&file);
        let (width, height) = get_map_size(&file);

        let mut unique_antinodes = HashSet::new();

        for (_, antenna) in frequencies {
            for a in &antenna {
                for b in &antenna {
                    if a == b {
                        continue;
                    }

                    let x = a.0 + (a.0 - b.0);
                    let y = a.1 + (a.1 - b.1);

                    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                        continue;
                    }

                    unique_antinodes.insert((x, y));
                }
            }
        }

        unique_antinodes.len()
    }

    fn run_star2(file: String) -> usize {
        let frequencies = parse_input(&file);
        let (width, height) = get_map_size(&file);

        let mut unique_antinodes = HashSet::new();

        for (_, antenna) in frequencies {
            for a in &antenna {
                for b in &antenna {
                    if a == b {
                        continue;
                    }

                    let x_offset = a.0 - b.0;
                    let y_offset = a.1 - b.1;

                    let mut x = a.0;
                    let mut y = a.1;

                    loop {
                        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                            break;
                        }

                        unique_antinodes.insert((x, y));

                        x += x_offset;
                        y += y_offset;
                    }
                }
            }
        }

        unique_antinodes.len()
    }
}

fn get_map_size(file: &str) -> (usize, usize) {
    let height = file.lines().count();
    let width = file.lines().next().unwrap().len();

    (height, width)
}

fn parse_input(file: &str) -> Vec<(char, Vec<(i32, i32)>)> {
    let mut map = HashMap::new();

    file.lines().enumerate().for_each(|(y, line)| {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            let Some(vec) = map.get_mut(&char) else {
                map.insert(char, vec![(x as i32, y as i32)]);
                continue;
            };

            vec.push((x as i32, y as i32));
        }
    });

    map.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::day8::Day8;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day8::run_star1(Day8::get_example());
        assert_eq!(result, 14);
    }

    #[test]
    fn example_star2() {
        let result = Day8::run_star2(Day8::get_example());
        assert_eq!(result, 34);
    }
}
