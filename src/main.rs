mod days;

use std::env;

fn execute_day(day: usize) -> Result<&'static str, &'static str> {
    match day {
        1 => Ok(days::day1::solve()),
        2 => Ok(days::day2::solve()),
        _ => Err("Oh no"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let output = execute_day(day).unwrap();

    println!("Part 1: {}", output);
}
