use crate::Day;

pub struct Day9 {}
impl Day<usize> for Day9 {
    fn number() -> usize {
        9
    }

    fn run_star1(file: String) -> usize {
        let mut vec = parse_input_expand(&file);

        loop {
            let (from_index, value) = vec
                .iter()
                .enumerate()
                .rfind(|(_, char)| !char.is_empty())
                .unwrap();
            let option = vec[0..from_index]
                .iter()
                .enumerate()
                .find(|(_, char)| char.is_empty());

            let Some((to_index, _)) = option else {
                break;
            };

            vec[to_index] = value.to_string();
            vec[from_index] = "".to_string()
        }

        vec.iter()
            .filter_map(|value| value.parse::<usize>().ok())
            .enumerate()
            .map(|(index, value)| index * value)
            .sum()
    }

    fn run_star2(file: String) -> usize {
        let mut vec = parse_input(&file);

        let mut index = 0;

        loop {
            if index > (vec.len() - 1) {
                break;
            }

            let from_index = vec.len() - 1 - index;
            index += 1;

            let (value, count) = &vec[from_index];
            if value.is_empty() {
                continue;
            }

            let option = vec[0..from_index]
                .iter()
                .enumerate()
                .find(|(_, (char, empty_count))| char.is_empty() && empty_count >= count);

            let Some((to_index, (_, empty_count))) = option else {
                continue;
            };

            let empty_count = *empty_count;
            let count = *count;
            let value = value.to_string();

            vec[from_index].0 = "".to_string();
            if empty_count == count {
                vec[to_index].0 = value;
            } else {
                vec[to_index].0 = value;
                vec[to_index].1 = count;

                vec.insert(to_index + 1, ("".to_string(), empty_count - count));
            }
        }

        let mut sum = 0;

        let mut index = 0;
        for (value, count) in vec {
            index += count;

            let value = value.parse::<usize>();
            let Ok(value) = value else {
                continue;
            };

            for x in (index - count)..index {
                sum += value * x;
            }
        }

        sum
    }
}
fn parse_input_expand(file: &str) -> Vec<String> {
    file.chars()
        .enumerate()
        .flat_map(|(index, c)| {
            let value = c.to_string().parse::<usize>().unwrap();

            if index % 2 == 0 {
                vec![(index / 2).to_string(); value]
            } else {
                vec!["".to_string(); value]
            }
        })
        .collect()
}

fn parse_input(file: &str) -> Vec<(String, usize)> {
    file.chars()
        .enumerate()
        .map(|(index, c)| {
            let value = c.to_string().parse::<usize>().unwrap();

            if index % 2 == 0 {
                ((index / 2).to_string(), value)
            } else {
                ("".to_string(), value)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day9::Day9;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day9::run_star1(Day9::get_example());
        assert_eq!(result, 1928);
    }

    #[test]
    fn example_star2() {
        let result = Day9::run_star2(Day9::get_example());
        assert_eq!(result, 2858);
    }
}
