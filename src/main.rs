use std::fs;

pub mod day1;
mod day2;

fn main() {
    println!("{}", day2::run_star2(read_file(2, false)));
}

pub fn read_file(day: usize, example: bool) -> String {
    fs::read_to_string(format!(
        "resources/{}/day{}.txt",
        if example { "example" } else { "input" },
        day
    ))
    .unwrap()
}
