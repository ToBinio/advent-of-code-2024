use crate::Day;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day5 {}
impl Day<usize> for Day5 {
    fn number() -> usize {
        5
    }

    fn run_star1(file: String) -> usize {
        let (manuals, rules) = parse_input(file);

        manuals
            .iter()
            .filter(|manual| is_valid_manual(*manual, &rules))
            .map(|manuel| manuel[manuel.len() / 2])
            .sum()
    }

    fn run_star2(file: String) -> usize {
        let (mut manuals, rules) = parse_input(file);

        manuals
            .iter_mut()
            .filter(|manual| !is_valid_manual(*manual, &rules))
            .map(|manual| {
                sort_manual(manual, &rules);
                manual
            })
            .map(|manuel| manuel[manuel.len() / 2])
            .sum()
    }
}

fn sort_manual(manual: &mut Vec<usize>, rules: &HashMap<usize, Vec<usize>>) {
    manual.sort_by(|a, b| {
        if let Some(after) = rules.get(a) {
            if after.contains(b) {
                return Ordering::Less;
            }
        }

        if let Some(after) = rules.get(b) {
            if after.contains(a) {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    })
}

fn is_valid_manual(manual: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    let manual = manual
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (index, value)| {
            map.insert(value, index);
            map
        });

    for (before, index) in manual.iter() {
        let Some(after) = rules.get(before) else {
            continue;
        };

        for after in after {
            let Some(after_index) = manual.get(after) else {
                continue;
            };

            if after_index < index {
                return false;
            }
        }
    }

    true
}

fn parse_input(file: String) -> (Vec<Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut rules = vec![];
    let mut manuels = vec![];

    let mut lines = file.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('|');

        let first: usize = split.next().unwrap().parse().unwrap();
        let second: usize = split.next().unwrap().parse().unwrap();

        rules.push((first, second));
    }

    while let Some(line) = lines.next() {
        let manuel: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();

        manuels.push(manuel);
    }

    let rules = rules
        .iter()
        .fold(HashMap::new(), |mut map, (before, after)| {
            match map.get_mut(before) {
                None => {
                    map.insert(*before, vec![*after]);
                }
                Some(value) => {
                    value.push(*after);
                }
            }

            map
        });

    (manuels, rules)
}

#[cfg(test)]
mod tests {
    use crate::day5::Day5;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day5::run_star1(Day5::get_example());
        assert_eq!(result, 143);
    }

    #[test]
    fn example_star2() {
        let result = Day5::run_star2(Day5::get_example());
        assert_eq!(result, 123);
    }
}
