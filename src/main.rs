use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;
use crate::day7::Day7;
use std::fmt::Debug;
use std::time::Instant;
use std::{env, fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<usize> = env::args()
        .skip(1)
        .map(|arg| arg.parse().unwrap())
        .collect();

    let instant = Instant::now();

    match (args[0], args[1]) {
        (1, star) => Day1::run(star),
        (2, star) => Day2::run(star),
        (3, star) => Day3::run(star),
        (4, star) => Day4::run(star),
        (5, star) => Day5::run(star),
        (6, star) => Day6::run(star),
        (7, star) => Day7::run(star),

        (day, star) => {
            panic!("day: {} - star:{} not jet done", day, star);
        }
    }

    println!("{:?}", instant.elapsed());
}
trait Day<T: Debug> {
    fn number() -> usize;

    fn run(star: usize) {
        match star {
            1 => println!("{:?}", Self::run_star1(Self::get_file())),
            2 => println!("{:?}", Self::run_star2(Self::get_file())),
            _ => panic!("day: {} - star:{} not jet done", Self::number(), star),
        }
    }
    fn run_star1(file: String) -> T;

    fn run_star2(file: String) -> T;

    fn get_file() -> String {
        read_file(Self::number(), false, 0)
    }

    #[allow(dead_code)]
    fn get_example() -> String {
        read_file(Self::number(), true, 0)
    }

    #[allow(dead_code)]
    fn get_example_part(part: usize) -> String {
        read_file(Self::number(), true, part)
    }
}

pub fn read_file(day: usize, example: bool, part: usize) -> String {
    fs::read_to_string(format!(
        "resources/{}/day{}{}.txt",
        if example { "example" } else { "input" },
        day,
        if part != 0 {
            format!(".{}", part)
        } else {
            "".to_string()
        },
    ))
    .unwrap()
}
