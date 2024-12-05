use crate::day4::Day4;
use crate::day5::Day5;
use std::fmt::{Debug, Display};
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("{}", Day5::run_star2(Day5::get_file()));
}
trait Day<T: Debug> {
    fn number() -> usize;
    fn run_star1(file: String) -> T;

    fn run_star2(file: String) -> T;

    fn get_file() -> String {
        read_file(Self::number(), false, 0)
    }

    fn get_example() -> String {
        read_file(Self::number(), true, 0)
    }

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
