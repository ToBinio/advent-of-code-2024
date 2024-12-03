use crate::Day;
use std::collections::HashMap;

pub struct Day1 {}

impl Day<usize> for Day1 {
    fn number() -> usize {
        1
    }

    fn run_star1(file: String) -> usize {
        let (list1, list2) = parse_inputs(file);

        list1
            .iter()
            .zip(list2.iter())
            .map(|(elem1, elem2)| elem2.abs_diff(*elem1))
            .sum()
    }

    fn run_star2(file: String) -> usize {
        let (list1, list2) = parse_inputs(file);

        let map = list2.iter().fold(HashMap::new(), |mut map, elem| {
            *map.entry(*elem).or_insert(0) += 1;
            map
        });

        list1
            .iter()
            .map(|elem| elem * map.get(elem).unwrap_or(&0))
            .sum()
    }
}

fn parse_inputs(file: String) -> (Vec<usize>, Vec<usize>) {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = file
        .lines()
        .map(|line| line.split("   "))
        .map(|mut split| {
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();

    list1.sort();
    list2.sort();

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use crate::day1::Day1;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day1::run_star1(Day1::get_example());
        assert_eq!(result, 11);
    }

    #[test]
    fn example_star2() {
        let result = Day1::run_star2(Day1::get_example());
        assert_eq!(result, 31);
    }
}
