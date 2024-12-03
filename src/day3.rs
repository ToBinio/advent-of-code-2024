use regex::Regex;

pub fn run_star1(file: String) -> usize {
    parse_muls(&file).iter().map(|(_, val)| val).sum()
}

pub fn run_star2(file: String) -> usize {
    let valid_ranges = build_on_ranges(&file);
    let muls = parse_muls(&file);

    let mut sum = 0;

    'outer: for (start, value) in muls {
        for (range_start, range_end) in &valid_ranges {
            if (*range_start..*range_end).contains(&start) {
                sum += value;
                continue 'outer;
            }
        }
    }

    sum
}

fn parse_muls(file: &str) -> Vec<(usize, usize)> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    regex
        .captures_iter(&file)
        .map(|capture| {
            (
                capture.get(0).unwrap().end(),
                capture.get(0).unwrap().as_str(),
            )
        })
        .map(|(start, str)| (start, str[4..str.len() - 1].to_string()))
        .map(|(start, str)| {
            let mut split = str.split(",");

            let first = split.next().unwrap().parse::<usize>().unwrap();
            let second = split.next().unwrap().parse::<usize>().unwrap();

            (start, first * second)
        })
        .collect()
}

fn build_on_ranges(file: &str) -> Vec<(usize, usize)> {
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();

    let dont_vec: Vec<_> = dont_regex
        .captures_iter(file)
        .map(|capture| capture.get(0).unwrap().end())
        .collect();

    let mut do_vec: Vec<_> = do_regex
        .captures_iter(file)
        .map(|capture| capture.get(0).unwrap().end())
        .collect();

    do_vec.insert(0, 0);

    let mut dont_iter = dont_vec.iter();
    let mut do_iter = do_vec.iter();

    let mut ranges = vec![];

    let mut last_do = do_iter.next().unwrap();
    let mut last_dont = dont_iter.next().unwrap();

    ranges.push((*last_do, *last_dont));

    'outer: loop {
        while last_do < last_dont {
            let next = do_iter.next();

            match next {
                None => {
                    break 'outer;
                }
                Some(next) => {
                    last_do = next;
                }
            }
        }

        while last_dont < last_do {
            let next = dont_iter.next();

            match next {
                None => {
                    ranges.push((*last_do, usize::MAX));
                    break 'outer;
                }
                Some(next) => {
                    last_dont = next;
                }
            }
        }

        ranges.push((*last_do, *last_dont));
    }

    ranges
}

#[cfg(test)]
mod tests {
    use crate::day3::{run_star1, run_star2};
    use crate::read_file;

    #[test]
    fn example_star1() {
        let result = run_star1(read_file(3, true, 0));
        assert_eq!(result, 161);
    }

    #[test]
    fn example_star2() {
        let result = run_star2(read_file(3, true, 2));
        assert_eq!(result, 48);
    }
}
