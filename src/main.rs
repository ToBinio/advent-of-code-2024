use std::fs;

pub mod day1;

fn main() {
    println!("{}", day1::run_star2(read_file(1, false)));
}

pub fn read_file(day: usize, example: bool) -> String {
    fs::read_to_string(format!(
        "resources/{}/day{}.txt",
        if example { "example" } else { "input" },
        day
    ))
    .unwrap()
}
