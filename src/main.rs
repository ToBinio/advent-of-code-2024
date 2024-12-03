use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    println!("{}", day3::run_star2(read_file(3, false, 0)));
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
