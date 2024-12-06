use crate::Day;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Not;

pub struct Day6 {}

#[derive(Clone)]
enum Tile {
    EMPTY,
    OBSTACLE,
}
impl Day<usize> for Day6 {
    fn number() -> usize {
        6
    }

    fn run_star1(file: String) -> usize {
        let (map, guard_pos) = parse_input(file);

        let visited_tiles = calc_path(&map, &guard_pos);

        visited_tiles.len()
    }

    fn run_star2(file: String) -> usize {
        let (map, guard_pos) = parse_input(file);

        let mut possible_tiles = calc_path(&map, &guard_pos);
        possible_tiles.remove(&guard_pos);

        possible_tiles
            .par_iter()
            .filter(|tile_to_block| {
                let mut map = map.clone();
                map[tile_to_block.1 as usize][tile_to_block.0 as usize] = Tile::OBSTACLE;

                is_loop(&map, &guard_pos)
            })
            .count()
    }
}

fn calc_path(map: &Vec<Vec<Tile>>, guard_pos: &(i32, i32)) -> HashSet<(i32, i32)> {
    let mut guard_pos = guard_pos.clone();
    let mut guard_direction = (0, -1);

    let mut visited_tiles = HashSet::new();
    visited_tiles.insert(guard_pos);

    loop {
        let next_guard_pos = (
            guard_pos.0 + guard_direction.0,
            guard_pos.1 + guard_direction.1,
        );

        if is_outside(map, next_guard_pos) {
            break;
        }

        if let Tile::OBSTACLE = map[next_guard_pos.1 as usize][next_guard_pos.0 as usize] {
            guard_direction = (-guard_direction.1, guard_direction.0);
            continue;
        }

        guard_pos = next_guard_pos;
        visited_tiles.insert(guard_pos);
    }
    visited_tiles
}

fn is_loop(map: &Vec<Vec<Tile>>, guard_pos: &(i32, i32)) -> bool {
    let mut guard_pos = guard_pos.clone();
    let mut guard_direction = (0, -1);

    let mut visited_tiles = HashSet::new();
    visited_tiles.insert((guard_pos, guard_direction));

    loop {
        let next_guard_pos = (
            guard_pos.0 + guard_direction.0,
            guard_pos.1 + guard_direction.1,
        );

        if is_outside(map, next_guard_pos) {
            break;
        }

        if let Tile::OBSTACLE = map[next_guard_pos.1 as usize][next_guard_pos.0 as usize] {
            guard_direction = (-guard_direction.1, guard_direction.0);
            continue;
        }

        guard_pos = next_guard_pos;

        if visited_tiles.insert((guard_pos, guard_direction)).not() {
            return true;
        }
    }

    false
}

fn is_outside(map: &Vec<Vec<Tile>>, next_guard_pos: (i32, i32)) -> bool {
    next_guard_pos.0 < 0
        || next_guard_pos.1 < 0
        || next_guard_pos.1 >= map.len() as i32
        || next_guard_pos.0 >= map[0].len() as i32
}

fn parse_input(file: String) -> (Vec<Vec<Tile>>, (i32, i32)) {
    let mut guard = (0, 0);

    let map = file
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Tile::EMPTY,
                    '#' => Tile::OBSTACLE,
                    '^' => {
                        guard = (x as i32, y as i32);
                        Tile::EMPTY
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (map, guard)
}

#[cfg(test)]
mod tests {
    use crate::day6::Day6;
    use crate::Day;

    #[test]
    fn example_star1() {
        let result = Day6::run_star1(Day6::get_example());
        assert_eq!(result, 41);
    }

    #[test]
    fn example_star2() {
        let result = Day6::run_star2(Day6::get_example());
        assert_eq!(result, 6);
    }
}
